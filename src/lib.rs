/// Api Docs: http://letterboxd-api.dev.cactuslab.com

extern crate crypto;
extern crate hex;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;
extern crate futures;
extern crate tokio_core;

use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;
use futures::{Future, Stream, future};
use hex::ToHex;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Handle;
use uuid::Uuid;

mod error;
mod defs;

pub use self::error::Error;
pub use defs::*;

fn nonce() -> Uuid {
    Uuid::new_v4()
}

fn now() -> u64 {
    let now = SystemTime::now();
    let dur = now.duration_since(UNIX_EPOCH).expect(
        "SystemTime::duration_since failed",
    );
    dur.as_secs()
}

fn hmac_sha256(secret: &str, msg: &str) -> String {
    let mut hmac = Hmac::new(Sha256::new(), secret.as_bytes());
    hmac.input(msg.as_bytes());
    hmac.result().code().to_hex()
}

pub struct Client {
    url: String,
    key: String,
    shared_secret: String,
    // state
    hyper_client: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
}

impl Client {
    pub fn new(handle: &Handle, api_key: String, api_shared_secret: String) -> Client {
        let hyper_client = hyper::Client::configure()
            .connector(HttpsConnector::new(4, handle).unwrap())
            .build(handle);
        Client {
            url: String::from("https://letterboxd.com/api/v0/"),
            key: api_key,
            shared_secret: api_shared_secret,
            hyper_client: hyper_client,
        }
    }

    pub fn auth(
        &self,
        username: &str,
        password: &str,
    ) -> Box<Future<Item = AccessToken, Error = Error>> {
        let body = format!("grant_type=password&username={}&password={}", username, password);
        let uri: hyper::Uri =
            match self.generate_signed_url(hyper::Method::Post, "auth/token", &vec![], &body)
                .parse() {
                Ok(uri) => uri,
                Err(err) => return Box::new(future::result(Err(Error::from(err)))),
            };

        let mut req = hyper::Request::new(hyper::Method::Post, uri.clone());
        req.headers_mut().set(
            hyper::header::ContentType::form_url_encoded(),
        );
        req.headers_mut().set(hyper::header::Accept::json());
        req.set_body(body);

        let patch = self.hyper_client.request(req).from_err();
        let fut_resp = patch.and_then(move |resp| {
            let status_code = resp.status();
            let body = resp.body().concat2().from_err();
            body.and_then(move |chunk| if status_code != hyper::StatusCode::Ok {
                let resp = String::from(str::from_utf8(&chunk)?);
                Err(Error::server_error(status_code, resp, uri))
            } else {
                let json: AccessToken = serde_json::from_slice(&chunk)?;
                Ok(json)
            })
        });
        Box::new(fut_resp)
    }

    fn generate_signed_url(
        &self,
        method: hyper::Method,
        endpoint: &str,
        parameters: &[(&'static str, String)],
        body: &str,
    ) -> String {
        self.url_with_nonce_and_timestamp(method, endpoint, parameters, body, nonce(), now())
    }

    fn url_with_nonce_and_timestamp(
        &self,
        method: hyper::Method,
        endpoint: &str,
        parameters: &[(&'static str, String)],
        body: &str,
        nonce: Uuid,
        timestamp: u64,
    ) -> String {
        let parameters: Vec<String> = parameters
            .iter()
            .map(|&(key, ref val)| format!("{}={}", key, val))
            .collect();
        let parameters = parameters.join("&");

        let url = format!(
            "{}{}?apikey={}&nonce={}&timestamp={}{}{}",
            self.url,
            endpoint,
            self.key,
            nonce,
            timestamp,
            if !parameters.is_empty() { "&" } else { "" },
            parameters
        );

        let salted_msg = format!("{}\0{}\0{}", method, url, body);
        format!("{}&signature={}", url, hmac_sha256(&self.shared_secret, &salted_msg))
    }

    pub fn search(
        &self,
        search_request: SearchRequest,
    ) -> Box<Future<Item = SearchResponse, Error = Error>> {
        let uri: hyper::Uri = match self.url_for_search(search_request).parse() {
            Ok(uri) => uri,
            Err(err) => {
                return Box::new(future::result(Err(Error::from(err))));
            }
        };
        let get = self.hyper_client.get(uri.clone()).from_err();
        let fut_resp = get.and_then(move |resp| {
            let status_code = resp.status();
            let body = resp.body().concat2().from_err();
            body.and_then(move |chunk| if status_code != hyper::StatusCode::Ok {
                let resp = String::from(str::from_utf8(&chunk)?);
                Err(Error::server_error(status_code, resp, uri))
            } else {
                let json: SearchResponse = serde_json::from_slice(&chunk)?;
                Ok(json)
            })
        });
        Box::new(fut_resp)
    }

    fn url_for_search(&self, search_request: SearchRequest) -> String {
        self.generate_signed_url(
            hyper::Method::Get,
            "search",
            &search_request.into_url_params(),
            "",
        )
    }

    pub fn patch_list(
        &self,
        id: &str,
        update_request: &ListUpdateRequest,
        access_token: &AccessToken,
    ) -> Box<Future<Item = ListUpdateResponse, Error = Error>> {
        let body = match serde_json::to_string(update_request) {
            Ok(body) => body,
            Err(err) => return Box::new(future::result(Err(Error::from(err)))),
        };
        let uri: hyper::Uri = match self.generate_signed_url(
            hyper::Method::Patch,
            &format!("list/{}", id),
            &vec![],
            &body,
        ).parse() {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::result(Err(Error::from(err)))),
        };

        let mut req = hyper::Request::new(hyper::Method::Patch, uri.clone());
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(
            body.len() as u64,
        ));
        req.headers_mut().set(hyper::header::Authorization(
            hyper::header::Bearer { token: access_token.access_token.clone() },
        ));
        req.set_body(body);

        let patch = self.hyper_client.request(req).from_err();
        let fut_resp = patch.and_then(move |resp| {
            let status_code = resp.status();
            let body = resp.body().concat2().from_err();
            body.and_then(move |chunk| if status_code != hyper::StatusCode::Ok {
                let resp = String::from(str::from_utf8(&chunk)?);
                Err(Error::server_error(status_code, resp, uri))
            } else {
                let json: ListUpdateResponse = serde_json::from_slice(&chunk)?;
                Ok(json)
            })
        });
        Box::new(fut_resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use tokio_core::reactor::Core;

    fn get_test_client() -> (Core, Client) {
        let core = Core::new().unwrap();
        let key = String::from("4a168ac5ef7f124d03364db8be04394f319a4114a2e70695fa585ef778dd15e6");
        let secret =
            String::from("27be8dfc7d2c27e8cffb0b74a8e5c9235e70c71f6c34892677bd6746fbcc0b0b");
        let client = Client::new(&core.handle(), key, secret);
        (core, client)
    }

    #[test]
    fn test_url() {
        let (_, lbd) = get_test_client();
        let uuid = Uuid::from_str("9d54386f-118e-4876-b8e8-92ba37d451e7")
            .expect("Uuid::from_str failed to parse example uuid.");
        let timestamp = 1499803866u64;
        assert_eq!(
            lbd.url_with_nonce_and_timestamp(
                hyper::Method::Get,
                "film/2a9q",
                &vec![("foo", String::from("bar"))],
                "",
                uuid,
                timestamp,
            ),
            "https://letterboxd.com/api/v0/film/2a9q?apikey=4a168ac5ef7f124d03364db8be04394f319a4114a2e70695fa585ef778dd15e6&nonce=9d54386f-118e-4876-b8e8-92ba37d451e7&timestamp=1499803866&foo=bar&signature=46fe62e84e3b3d417cb539a9d3a5ea79f51f37cc5311d4583ef7d1f9444f8797"
        );
    }

    #[test]
    fn test_url_for_search() {
        let (_, lbd) = get_test_client();

        let url = lbd.url_for_search(SearchRequest::new(String::from("foobar")));
        assert!(url.contains("input=foobar"));

        let mut search_request = SearchRequest::new(String::from("Fight Club"));
        search_request.search_method = Some(SearchMethod::FullText);
        let url = lbd.url_for_search(search_request);
        assert!(url.contains("input=Fight+Club"));
        assert!(url.contains("searchMethod=FullText"));

        let mut search_request = SearchRequest::new(String::from("Fight Club"));
        search_request.include = Some(vec![
            SearchResultType::FilmSearchItem,
            SearchResultType::ListSearchItem,
        ]);
        let url = lbd.url_for_search(search_request);
        assert!(url.contains("input=Fight+Club"));
        assert!(url.contains("include=FilmSearchItem&include=ListSearchItem"));

        let mut search_request = SearchRequest::new(String::from("Fight Club"));
        search_request.per_page = Some(100);
        let url = lbd.url_for_search(search_request);
        assert!(url.contains("input=Fight+Club"));
        assert!(url.contains("perPage=100"));

        let mut search_request = SearchRequest::new(String::from("Fight Club"));
        search_request.cursor = Some(String::from("some-unique-id"));
        let url = lbd.url_for_search(search_request);
        assert!(url.contains("input=Fight+Club"));
        assert!(url.contains("cursor=some-unique-id"));

        let search_request = SearchRequest::new(String::from("Брат"));
        let url = lbd.url_for_search(search_request);
        assert!(url.contains("input=Брат"));
    }
}
