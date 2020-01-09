use crate::defs;
use crate::error::{Error, Result};
use crate::helper;

use futures::stream::StreamExt;
use hyper::{
    client::HttpConnector,
    header::{self, HeaderValue},
    Body, Method, Request, StatusCode, Uri,
};
use hyper_tls::HttpsConnector;
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

/// API key/secret pair.
///
/// Can be created explicitly, or from default environment variables
/// `LETTERBOXD_API_KEY` and `LETTERBOXD_API_SECRET`.
#[derive(Debug, Clone)]
pub struct ApiKeyPair {
    api_key: String,
    api_secret: String,
}

impl ApiKeyPair {
    /// Environment variable name used to get API key.
    pub const API_KEY_ENVVAR: &'static str = "LETTERBOXD_API_KEY";
    /// Environment variable name used to get API secret.
    pub const API_SECRET_ENVVAR: &'static str = "LETTERBOXD_API_SECRET";

    /// Creates new ApiKeyPair from given key and secret.
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_key,
            api_secret,
        }
    }

    /// Tries to create an new api key pair from environment.
    ///
    /// The environment variable name are defined by constants `API_KEY_ENVVAR`
    /// and `API_SECRET_ENVVAR`.
    ///
    /// If one of the variables is missing, returns `None`.
    pub fn from_env() -> Option<Self> {
        match (
            std::env::var(Self::API_KEY_ENVVAR),
            std::env::var(Self::API_SECRET_ENVVAR),
        ) {
            (Ok(api_key), Ok(api_secret)) => Some(Self::new(api_key, api_secret)),
            _ => None,
        }
    }
}

/// Letterboxd asynchronous client.
///
/// Client is created from given api key pair either
///
/// * by authenticating using a username/password,
/// * with a token (all API calls will be authenticated),
/// * without a token (no API calls will be authenticated; calls that require
///   authentication will fail).
///
/// **Note**: Not all APIs are implemented. Feel free to contribute implementation for missing
/// endpoints. The implementation is usually very straight forward.
pub struct Client {
    api_key_pair: ApiKeyPair,
    token: Option<defs::AccessToken>,
    http_client: hyper::Client<HttpsConnector<HttpConnector>>,
}

impl Client {
    const API_BASE_URL: &'static str = "https://api.letterboxd.com/api/v0/";

    /// Creates a new client without authentication.
    pub fn new(api_key_pair: ApiKeyPair) -> Self {
        let https = hyper_tls::HttpsConnector::new();
        let http_client = hyper::Client::builder().build::<_, Body>(https);

        Self {
            api_key_pair,
            token: None,
            http_client,
        }
    }

    /// Crates a new client from a given token.
    ///
    /// It is not checked that the token is valid.
    pub fn with_token(api_key_pair: ApiKeyPair, token: defs::AccessToken) -> Self {
        let https = HttpsConnector::new();
        let http_client = hyper::Client::builder().build::<_, Body>(https);
        Self {
            api_key_pair,
            token: Some(token),
            http_client,
        }
    }

    /// Authenticates and creates a new client from given username/password.
    pub async fn authenticate(
        api_key_pair: ApiKeyPair,
        username: &str,
        password: &str,
    ) -> Result<Self> {
        let https = hyper_tls::HttpsConnector::new();
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
            token: Some(token),
            http_client,
        })
    }

    /// Returns if the client has a token.
    ///
    /// This method does *not* check that the token is valid.
    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }

    /// Returns the token used for authentication.
    pub fn token(&self) -> Option<&defs::AccessToken> {
        self.token.as_ref()
    }

    /// Sets a new token which will be used for authentication.
    ///
    /// Setting `None` disables authentication.
    pub fn set_token(&mut self, token: Option<defs::AccessToken>) {
        self.token = token;
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
    pub async fn delete_list(&self, id: &str) -> Result<()> {
        self.delete(&format!("list/{}", id)).await
    }

    //     /list/{id}/comments

    /// Get entries for a list by ID.
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
        self.request::<(), (), _>(Method::GET, endpoint_path, None, None)
            .await
    }

    async fn get_with_query<Q, R>(&self, endpoint_path: &str, query: &Q) -> Result<R>
    where
        Q: Serialize,
        R: DeserializeOwned + 'static,
    {
        self.request::<_, (), _>(Method::GET, endpoint_path, Some(query), None)
            .await
    }

    async fn patch<B, R>(&self, endpoint_path: &str, body: &B) -> Result<R>
    where
        B: Serialize,
        R: DeserializeOwned + 'static,
    {
        self.request::<(), _, _>(Method::PATCH, endpoint_path, None, Some(body))
            .await
    }

    async fn post<B, R>(&self, endpoint_path: &str, body: &B) -> Result<R>
    where
        B: Serialize,
        R: DeserializeOwned + 'static,
    {
        self.request::<(), _, _>(Method::POST, endpoint_path, None, Some(body))
            .await
    }

    async fn delete(&self, endpoint_path: &str) -> Result<()> {
        self.request_bytes::<(), ()>(Method::DELETE, endpoint_path, None, None)
            .await?;
        Ok(())
    }

    async fn request<Q, B, R>(
        &self,
        method: Method,
        endpoint_path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<R>
    where
        Q: Serialize,
        B: Serialize,
        R: DeserializeOwned + 'static,
    {
        let bytes = self
            .request_bytes(method, endpoint_path, query, body)
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
    ) -> Result<Vec<u8>>
    where
        Q: Serialize,
        B: Serialize,
    {
        let mut url = Url::parse(Self::API_BASE_URL)
            .unwrap()
            .join(endpoint_path)
            .unwrap(); // TODO
        let query = query.map(serde_url_params::to_string).transpose()?;
        url.set_query(query.as_ref().map(|s| s.as_ref()));
        let body = body
            .map(serde_json::to_vec)
            .transpose()?
            .unwrap_or_default();

        let signed_url = self.sign_url(url, &method, &body);

        let req = Request::builder()
            .method(method)
            .uri(signed_url.as_str())
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            )
            .header(
                header::CONTENT_LENGTH,
                HeaderValue::from_str(&format!("{}", body.len())).expect("invalid header value"),
            );

        let req = if let Some(token) = self.token.as_ref() {
            req.header(
                header::AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", token.access_token))
                    .expect("invalid header value"),
            )
        } else {
            req
        };

        let req = req.body(Body::from(body)).expect("invalid body");
        let resp = self.http_client.request(req).await?;
        let status = resp.status();

        let mut body = resp.into_body();
        let mut bytes = Vec::new();
        while let Some(next) = body.next().await {
            let chunk = next?;
            bytes.extend(chunk);
        }

        if !status.is_success() {
            let content = String::from_utf8_lossy(&bytes);
            return Err(Error::server_error(
                status,
                content.to_string(),
                signed_url.as_str().parse()?,
            ));
        }

        Ok(bytes)
    }

    /// Signs the request based on a random and unique nonce, timestamp, and
    /// client id and secret.
    ///
    /// The client id, nonce, timestamp and signature are added to the url's
    /// query.
    ///
    /// See http://api-docs.letterboxd.com/#signing.
    fn sign_url(&self, mut url: Url, method: &Method, body: &[u8]) -> Url {
        use crypto::{hmac, mac::Mac, sha2};
        use hex::ToHex;
        use std::time;

        let nonce = uuid::Uuid::new_v4(); // use UUID as random and unique nonce

        let timestamp = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .expect("SystemTime::duration_since failed")
            .as_secs();

        url.query_pairs_mut()
            .append_pair("apikey", &self.api_key_pair.api_key)
            .append_pair("nonce", &format!("{}", nonce))
            .append_pair("timestamp", &format!("{}", timestamp));

        // create signature
        let mut hmac =
            hmac::Hmac::new(sha2::Sha256::new(), self.api_key_pair.api_secret.as_bytes());
        hmac.input(method.as_str().as_bytes());
        hmac.input(&[b'\0']);
        hmac.input(url.as_str().as_bytes());
        hmac.input(&[b'\0']);
        hmac.input(body);
        let signature: String = hmac.result().code().encode_hex();

        url.query_pairs_mut().append_pair("signature", &signature);

        url
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
        if !parameters.unwrap_or_default().is_empty() {
            "&"
        } else {
            ""
        },
        parameters.unwrap_or_default()
    );

    let salted_msg = format!("{}\0{}\0{}", method, url, body.unwrap_or_default());
    format!(
        "{}&signature={}",
        url,
        helper::hmac_sha256(&api_key_pair.api_secret, &salted_msg)
    )
}
