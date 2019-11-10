use crate::defs;
use crate::error::{Error, Result};
use crate::helper;

use http::{header, HeaderValue, Method, StatusCode, Uri};
use hyper::{client::HttpConnector, Body, Request};
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
    ) -> Result<Self> {
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

    // film

    /// A cursored window over the list of films.
    ///
    /// Use the ‘next’ cursor to move through the list. The response will include the film
    /// relationships for the signed-in member and the member indicated by the member LID if
    /// specified.
    pub async fn films(&self, request: &defs::FilmsRequest) -> Result<defs::FilmsResponse> {
        self.get_with_query("films", request).await
    }

    /// Get a list of services supported by the /films endpoint.
    ///
    /// Services are returned in alphabetical order. Some services are only available to paying
    /// members, so results will vary based on the authenticated member’s status.
    pub async fn film_services(&self) -> Result<defs::FilmServicesResponse> {
        self.get("films/film-services").await
    }

    /// Get a list of genres supported by the `films` function.
    ///
    /// Genres are returned in alphabetical order.
    pub async fn film_genres(&self) -> Result<defs::GenresResponse> {
        self.get("films/genres").await
    }

    /// Get details about a film by ID.
    pub async fn film(&self, id: &str) -> Result<defs::Film> {
        self.get(&format!("film/{}", id)).await
    }

    /// Get availability data about a film by ID.
    pub async fn film_availability(&self, id: &str) -> Result<defs::FilmAvailabilityResponse> {
        self.get(&format!("film/{}/availability", id)).await
    }

    /// Get details of the authenticated member’s relationship with a film by ID.
    pub async fn film_relationship(&self, id: &str) -> Result<defs::FilmAvailabilityResponse> {
        self.get(&format!("film/{}/me", id)).await
    }

    /// Update the authenticated member’s relationship with a film by ID.
    pub async fn update_film_relationship(
        &self,
        id: &str,
        request: &defs::FilmRelationshipUpdateRequest,
    ) -> Result<defs::FilmRelationshipUpdateResponse> {
        self.patch(&format!("film/{}/me", id), request).await
    }

    /// Get details of the authenticated member’s relationship with a film by ID.
    pub async fn film_relationship_members(
        &self,
        id: &str,
        request: &defs::MemberFilmRelationshipsRequest,
    ) -> Result<defs::MemberFilmRelationshipsResponse> {
        self.get_with_query(&format!("film/{}/members", id), request)
            .await
    }

    //     /film/{id}/report

    /// Get statistical data about a film by ID.
    pub async fn film_statistics(&self, id: &str) -> Result<defs::FilmStatistics> {
        self.get(&format!("film/{}/statistics", id)).await
    }

    // list

    /// A cursored window over a list of lists.
    ///
    /// Use the ‘next’ cursor to move through the list.
    pub async fn lists(&self, request: &defs::ListsRequest) -> Result<defs::ListsResponse> {
        self.get_with_query("lists", request).await
    }

    /// Create a list.
    pub async fn create_list(
        &self,
        request: &defs::ListCreationRequest,
    ) -> Result<defs::ListCreateResponse> {
        self.post("lists", request).await
    }

    /// Get details of a list by ID.
    pub async fn list(&self, id: &str) -> Result<defs::List> {
        self.get(&format!("list/{}", id)).await
    }

    /// Update a list by ID.
    pub async fn update_list(
        &self,
        id: &str,
        request: &defs::ListUpdateRequest,
    ) -> Result<defs::ListUpdateResponse> {
        self.patch(&format!("list/{}", id), request).await
    }

    /// Delete a list by ID.
    pub async fn delete_list(&self, id: &str, request: &defs::ListUpdateRequest) -> Result<()> {
        self.delete(&format!("list/{}", id), request).await
    }

    //     /list/{id}/comments

    //     /list/{id}/entries

    /// Delete a list by ID.
    pub async fn list_entries(
        &self,
        id: &str,
        request: &defs::ListEntriesRequest,
    ) -> Result<defs::ListEntriesResponse> {
        self.get_with_query(&format!("list/{}/entries", id), request)
            .await
    }

    //     /list/{id}/me

    //     /list/{id}/report
    //     /list/{id}/statistics

    // log-entry

    //     /log-entries
    //     /log-entry/{id}
    //     /log-entry/{id}/comments
    //     /log-entry/{id}/me
    //     /log-entry/{id}/report
    //     /log-entry/{id}/statistics

    // me

    //     /me
    //     /me/validation-request

    // member

    //     /members
    //     /members/pronouns
    //     /members/register
    //     /member/{id}
    //     /member/{id}/activity
    //     /member/{id}/list-tags
    //     /member/{id}/list-tags-2
    //     /member/{id}/log-entry-tags
    //     /member/{id}/me
    //     /member/{id}/report
    //     /member/{id}/review-tags
    //     /member/{id}/review-tags-2
    //     /member/{id}/statistics
    //     /member/{id}/watchlist

    // search

    /// Search for any data.
    pub async fn search(&self, request: &defs::SearchRequest) -> Result<defs::SearchResponse> {
        self.get_with_query("search", request).await
    }

    // helper methods

    // request helper

    async fn get<R>(&self, endpoint_path: &str) -> Result<R>
    where
        R: DeserializeOwned + 'static,
    {
        self.request::<(), (), _>(Method::GET, endpoint_path, None, None, false)
            .await
    }

    async fn get_with_query<Q, R>(&self, endpoint_path: &str, query: &Q) -> Result<R>
    where
        Q: Serialize,
        R: DeserializeOwned + 'static,
    {
        self.request::<_, (), _>(Method::GET, endpoint_path, Some(query), None, false)
            .await
    }

    async fn patch<B, R>(&self, endpoint_path: &str, body: &B) -> Result<R>
    where
        B: Serialize,
        R: DeserializeOwned + 'static,
    {
        self.request::<(), _, _>(Method::GET, endpoint_path, None, Some(body), false)
            .await
    }

    async fn post<B, R>(&self, endpoint_path: &str, body: &B) -> Result<R>
    where
        B: Serialize,
        R: DeserializeOwned + 'static,
    {
        self.request::<(), _, _>(Method::POST, endpoint_path, None, Some(body), false)
            .await
    }

    async fn delete<B>(&self, endpoint_path: &str, body: &B) -> Result<()>
    where
        B: Serialize,
    {
        self.request_bytes::<(), _>(Method::POST, endpoint_path, None, Some(body), false)
            .await?;
        Ok(())
    }

    async fn request<Q, B, R>(
        &self,
        method: Method,
        endpoint_path: &str,
        query: Option<&Q>,
        body: Option<&B>,
        with_auth: bool,
    ) -> Result<R>
    where
        Q: Serialize,
        B: Serialize,
        R: DeserializeOwned + 'static,
    {
        let bytes = self
            .request_bytes(method, endpoint_path, query, body, with_auth)
            .await?;
        let res = serde_json::from_slice(&bytes)?;
        Ok(res)
    }

    async fn request_bytes<Q, B>(
        &self,
        method: Method,
        endpoint_path: &str,
        query: Option<&Q>,
        body: Option<&B>,
        with_auth: bool,
    ) -> Result<Vec<u8>>
    where
        Q: Serialize,
        B: Serialize,
    {
        let query = query.map(serde_url_params::to_string).transpose()?;
        let body = body.map(serde_json::to_string).transpose()?;

        let signed_url = generate_signed_url(
            method.clone(),
            endpoint_path,
            query.as_ref().map(String::as_str),
            body.as_ref().map(String::as_str),
            &self.api_key_pair,
        );

        let uri: Uri = signed_url.parse()?;

        let mut req = Request::builder();
        let req = req.method(method).uri(uri.clone()).header(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        let content_length = if let Some(body) = body.as_ref() {
            let content_length = body.as_bytes().len();
            HeaderValue::from_str(&format!("{}", content_length)).expect("invalid header value")
        } else {
            HeaderValue::from_static("0")
        };
        req.header(header::CONTENT_LENGTH, content_length);

        if with_auth {
            req.header(
                header::AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.token.access_token))
                    .expect("invalid header value"),
            );
        }

        let req = req
            .body(body.map(Body::from).unwrap_or_default())
            .expect("invalid body");
        let resp = self.http_client.request(req).await?;
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

        Ok(bytes)
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::env;
//     use tokio::runtime::current_thread::Runtime;

//     #[test]
//     fn test_list() -> Result<()> {
//         let api_key = env::var("API_KEY").expect("missing API_KEY");
//         let api_secret = env::var("API_SECRET").expect("missing API_SECRET");

//         let client = Client::new(ApiKeyPair::new(api_key, api_secret));
//         let req = defs::FilmsRequest {
//             per_page: Some(1),
//             ..Default::default()
//         };
//         let fut = client.films(&req);

//         let mut rt = Runtime::new().expect("runtime new");
//         let resp = rt.block_on(fut)?;
//         println!("{:?}", resp);

//         Ok(())
//     }
// }
