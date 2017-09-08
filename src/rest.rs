use std::str;

use futures::{Future, Stream, future};
use hyper;
use hyper_tls;
use serde_json;
use tokio_core::reactor::Handle;
use uuid;

use defs;
use error::Error;
use helper;

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
            .connector(hyper_tls::HttpsConnector::new(4, handle).unwrap())
            .build(handle);
        Client {
            url: String::from("https://letterboxd.com/api/v0/"),
            key: api_key,
            shared_secret: api_shared_secret,
            hyper_client: hyper_client,
        }
    }

    GET!(search, "search", defs::SearchRequest, defs::SearchResponse);

    GET!(lists, "lists", defs::ListsRequest, defs::ListsResponse);
    POST!(post_list, "lists", defs::ListCreationRequest, defs::ListCreateResponse);

    GET!(get_list, ("list/{}", id: &str), defs::List);
    PATCH!(patch_list, ("list/{}", id: &str), defs::ListUpdateRequest, defs::ListUpdateResponse);
    DELETE!(delete_list, ("list/{}", id: &str));

    pub fn auth(
        &self,
        username: &str,
        password: &str,
    ) -> Box<Future<Item = defs::AccessToken, Error = Error>> {
        let body = format!("grant_type=password&username={}&password={}", username, password);
        let uri: hyper::Uri =
            match self.generate_signed_url(hyper::Method::Post, "auth/token", "", &body)
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

        let req = self.hyper_client.request(req).from_err();
        let fut_resp = req.and_then(move |resp| {
            let status_code = resp.status();
            let body = resp.body().concat2().from_err();
            body.and_then(move |chunk| if status_code != hyper::StatusCode::Ok {
                let resp = String::from(str::from_utf8(&chunk)?);
                Err(Error::server_error(status_code, resp, uri))
            } else {
                let json: defs::AccessToken = serde_json::from_slice(&chunk)?;
                Ok(json)
            })
        });
        Box::new(fut_resp)
    }

    fn generate_signed_url(
        &self,
        method: hyper::Method,
        endpoint: &str,
        parameters: &str,
        body: &str,
    ) -> String {
        self.url_with_nonce_and_timestamp(
            method,
            endpoint,
            parameters,
            body,
            helper::nonce(),
            helper::now(),
        )
    }

    fn url_with_nonce_and_timestamp(
        &self,
        method: hyper::Method,
        endpoint: &str,
        parameters: &str,
        body: &str,
        nonce: uuid::Uuid,
        timestamp: u64,
    ) -> String {
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
        format!("{}&signature={}", url, helper::hmac_sha256(&self.shared_secret, &salted_msg))
    }
}
