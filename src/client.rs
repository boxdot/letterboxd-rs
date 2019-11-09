use crate::defs;
use crate::error::Error;
use crate::helper;

use futures::{future, Future, Stream};
use http::{header, HeaderValue, Method, Uri};
use hyper::{client::HttpConnector, Body};
use hyper_tls::HttpsConnector;

use std::str;

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

    pub fn authenticate(
        api_key_pair: ApiKeyPair,
        username: &str,
        password: &str,
    ) -> impl Future<Item = Self, Error = Error> {
        let https = hyper_tls::HttpsConnector::new(4).unwrap();
        let http_client = hyper::Client::builder().build::<_, Body>(https);

        let body = format!(
            "grant_type=password&username={}&password={}",
            username, password
        );
        let signed_url = generate_signed_url(Method::POST, "auth/token", "", &body, &api_key_pair);
        let uri: Uri = match signed_url.parse() {
            Ok(uri) => uri,
            Err(err) => return future::Either::A(future::err(Error::from(err))),
        };

        let req = hyper::Request::post(uri.clone())
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            )
            .header(header::ACCEPT, HeaderValue::from_static("application/json"))
            .body(Body::from(body))
            .unwrap();

        let req = http_client.request(req).from_err();
        let fut_auth_token = req.and_then(move |resp| {
            let status_code = resp.status();
            let body = resp.into_body().concat2().from_err();
            body.and_then(move |chunk| {
                if status_code != hyper::StatusCode::OK {
                    let resp = String::from(str::from_utf8(&chunk)?);
                    Err(Error::server_error(status_code, resp, uri))
                } else {
                    let json: defs::AccessToken = serde_json::from_slice(&chunk)?;
                    Ok(json)
                }
            })
        });
        let fut_client = fut_auth_token.and_then(|token| {
            Ok(Self {
                api_key_pair,
                token,
                http_client,
            })
        });
        future::Either::B(fut_client)
    }

    pub fn with_token(api_key_pair: ApiKeyPair, token: defs::AccessToken) -> Self {
        let https = hyper_tls::HttpsConnector::new(4).unwrap();
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
}

fn generate_signed_url(
    method: Method,
    endpoint: &str,
    parameters: &str,
    body: &str,
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
    parameters: &str,
    body: &str,
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
        if !parameters.is_empty() { "&" } else { "" },
        parameters
    );

    let salted_msg = format!("{}\0{}\0{}", method, url, body);
    format!(
        "{}&signature={}",
        url,
        helper::hmac_sha256(&api_key_pair.api_secret, &salted_msg)
    )
}
