use crate::defs;
use crate::error::Error;
use crate::helper;
use futures::{future, Future};

use http::{header, HeaderValue, Method, StatusCode, Uri};
use hyper::{client::HttpConnector, Body};
use hyper_tls::HttpsConnector;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone)]
pub struct ApiKeyPair {
    api_key: String,
    api_secret: String,
}

impl ApiKeyPair {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_key,
            api_secret,
        }
    }
}

pub struct Client {
    api_key_pair: ApiKeyPair,
    token: defs::AccessToken,
    http_client: hyper::Client<HttpsConnector<HttpConnector>>,
}

impl Client {
    const API_BASE_URL: &'static str = "https://api.letterboxd.com/api/v0/";

    pub fn new(api_key_pair: ApiKeyPair) -> Self {
        let https = hyper_tls::HttpsConnector::new().unwrap();
        let http_client = hyper::Client::builder().build::<_, Body>(https);

        Self {
            api_key_pair,
            token: Default::default(), // invalid token
            http_client,
        }
    }

    pub async fn authenticate(
        api_key_pair: ApiKeyPair,
        username: &str,
        password: &str,
    ) -> Result<Self, Error> {
        let https = hyper_tls::HttpsConnector::new().unwrap();
        let http_client = hyper::Client::builder().build::<_, Body>(https);

        let body = format!(
            "grant_type=password&username={}&password={}",
            username, password
        );
        let signed_url =
            generate_signed_url(Method::POST, "auth/token", None, Some(&body), &api_key_pair);
        let uri: Uri = signed_url.parse()?;

        let req = hyper::Request::post(uri.clone())
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            )
            .header(header::ACCEPT, HeaderValue::from_static("application/json"))
            .body(Body::from(body))
            .unwrap();

        let resp = http_client.request(req).await?;
        let status = resp.status();

        let mut body = resp.into_body();
        let mut bytes = Vec::new();
        while let Some(next) = body.next().await {
            let chunk = next?;
            bytes.extend(chunk);
        }

        if status != StatusCode::OK {
            let content = String::from_utf8_lossy(&bytes);
            return Err(Error::server_error(status, content.to_string(), uri));
        }

        let token: defs::AccessToken = serde_json::from_slice(&bytes)?;
        Ok(Self {
            api_key_pair,
            token,
            http_client,
        })
    }

    pub fn with_token(api_key_pair: ApiKeyPair, token: defs::AccessToken) -> Self {
        let https = HttpsConnector::new().unwrap();
        let http_client = hyper::Client::builder().build::<_, Body>(https);
        Self {
            api_key_pair,
            token,
            http_client,
        }
    }

    pub fn token(&self) -> &defs::AccessToken {
        &self.token
    }

    // API endpoints

    /// A cursored window over the list of films.
    ///
    /// Use the ‘next’ cursor to move through the list. The response will include the film
    /// relationships for the signed-in member and the member indicated by the member LID if
    /// specified.
    pub fn films(
        &self,
        request: &defs::FilmsRequest,
    ) -> impl Future<Output = Result<defs::FilmsResponse, Error>> + 'static {
        self.get("films", Some(request))
    }

    /// Get a list of services supported by the /films endpoint.
    ///
    /// Services are returned in alphabetical order. Some services are only available to paying
    /// members, so results will vary based on the authenticated member’s status.
    pub fn film_services(&self) -> impl Future<Output = Result<defs::FilmServicesResponse, Error>> {
        self.get::<(), _>("films/film-services", None)
    }

    /// Get a list of genres supported by the `films` function.
    ///
    /// Genres are returned in alphabetical order.
    pub fn film_genres(&self) -> impl Future<Output = Result<defs::GenresResponse, Error>> {
        self.get::<(), _>("films/genres", None)
    }

    /// Get details about a film by ID.
    pub fn film(&self, id: &str) -> impl Future<Output = Result<defs::Film, Error>> {
        self.get::<(), _>(&format!("film/{}", id), None)
    }

    /// Get availability data about a film by ID.
    pub fn film_availability(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<defs::FilmAvailabilityResponse, Error>> {
        self.get::<(), _>(&format!("film/{}/availability", id), None)
    }

    /// Get details of the authenticated member’s relationship with a film by ID.
    pub fn film_relationship(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<defs::FilmAvailabilityResponse, Error>> {
        self.get::<(), _>(&format!("film/{}/me", id), None)
    }

    // helper

    fn get<Q, R>(
        &self,
        endpoint_path: &str,
        query: Option<&Q>,
    ) -> impl Future<Output = Result<R, Error>> + 'static
    where
        Q: Serialize,
        R: DeserializeOwned + 'static,
    {
        let query = query
            .map(serde_url_params::to_string)
            .transpose()
            .map_err(Error::from);
        let query = match query {
            Ok(query) => query,
            Err(e) => return future::Either::Right(future::err(e)),
        };

        let signed_url = generate_signed_url(
            Method::GET,
            endpoint_path,
            query.as_ref().map(|s| s.as_str()),
            None,
            &self.api_key_pair,
        );

        let uri: Uri = match signed_url.parse().map_err(Error::from) {
            Ok(uri) => uri,
            Err(e) => return future::Either::Right(future::err(Error::from(e))),
        };

        let http_client = self.http_client.clone();

        let fut_resp = async move {
            let resp = http_client.get(uri.clone()).await?;
            let status = resp.status();

            let mut body = resp.into_body();
            let mut bytes = Vec::new();
            while let Some(next) = body.next().await {
                let chunk = next?;
                bytes.extend(chunk);
            }

            if status != StatusCode::OK {
                let content = String::from_utf8_lossy(&bytes);
                return Err(Error::server_error(status, content.to_string(), uri));
            }

            let response = serde_json::from_slice(&bytes)?;
            Ok(response)
        };

        future::Either::Left(fut_resp)
    }
}

fn generate_signed_url(
    method: Method,
    endpoint: &str,
    parameters: Option<&str>,
    body: Option<&str>,
    api_key_pair: &ApiKeyPair,
) -> String {
    url_with_nonce_and_timestamp(
        method,
        endpoint,
        parameters,
        body,
        api_key_pair,
        helper::nonce(),
        helper::now(),
    )
}

fn url_with_nonce_and_timestamp(
    method: Method,
    endpoint: &str,
    parameters: Option<&str>,
    body: Option<&str>,
    api_key_pair: &ApiKeyPair,
    nonce: uuid::Uuid,
    timestamp: u64,
) -> String {
    let url = format!(
        "{}{}?apikey={}&nonce={}&timestamp={}{}{}",
        Client::API_BASE_URL,
        endpoint,
        api_key_pair.api_key,
        nonce,
        timestamp,
        parameters.map(|_| "&").unwrap_or_default(),
        parameters.unwrap_or_default()
    );

    let salted_msg = format!("{}\0{}\0{}", method, url, body.unwrap_or_default());
    format!(
        "{}&signature={}",
        url,
        helper::hmac_sha256(&api_key_pair.api_secret, &salted_msg)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tokio::runtime::current_thread::Runtime;

    #[test]
    fn test_list() -> Result<(), Error> {
        let api_key = env::var("API_KEY").expect("missing API_KEY");
        let api_secret = env::var("API_SECRET").expect("missing API_SECRET");

        let fut = {
            let client = Client::new(ApiKeyPair::new(api_key, api_secret));
            let req = defs::FilmsRequest {
                per_page: Some(1),
                ..Default::default()
            };
            client.films(&req)
        };

        let mut rt = Runtime::new().expect("runtime new");
        let resp = rt.block_on(fut)?;
        println!("{:?}", resp);

        Ok(())
    }
}
