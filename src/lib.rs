/// Api Docs: http://letterboxd-api.dev.cactuslab.com

extern crate crypto;
extern crate hex;
extern crate hyper;
extern crate uuid;

use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;
use hex::ToHex;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum SearchMethod {
    FullText,
    Autocomplete,
}

#[derive(Debug, Clone)]
pub enum SearchResultType {
    ContributorSearchItem,
    FilmSearchItem,
    ListSearchItem,
    MemberSearchItem,
    ReviewSearchItem,
    TagSearchItem,
}

#[derive(Debug, Clone)]
pub enum ContributionType {
    Director,
    Actor,
    Producer,
    Writer,
    Editor,
    Cinematography,
    ArtDirection,
    VisualEffects,
    Composer,
    Sound,
    Costumes,
    MakeUp,
    Studio,
}

/// A cursor is a string value provided by the API. It should be treated as an
/// opaque value — don't change it.
pub type Cursor = String;

#[derive(Debug, Clone)]
pub struct SearchRequest {
    cursor: Option<Cursor>,
    per_page: Option<usize>,
    input: String,
    search_method: Option<SearchMethod>,
    include: Option<Vec<SearchResultType>>,
    contribution_type: Option<ContributionType>,
}

impl SearchRequest {
    pub fn new(input: String) -> SearchRequest {
        SearchRequest {
            cursor: None,
            per_page: None,
            input: input,
            search_method: None,
            include: None,
            contribution_type: None,
        }
    }

    fn into_url_params(self) -> Vec<(&'static str, String)> {
        // TODO: write a generic version for any serializable type
        let mut params: Vec<(&'static str, String)> =
            vec![
                ("cursor", self.cursor),
                ("perPage", self.per_page.as_ref().map(|x| x.to_string())),
                ("input", Some(self.input.replace(" ", "+"))),
                ("searchMethod", self.search_method.map(|x| format!("{:?}", x))),
                ("contributionType", self.contribution_type.map(|x| format!("{:?}", x))),
            ].into_iter()
                .filter_map(|(k, v)| if let Some(v) = v { Some((k, v)) } else { None })
                .collect();

        if let Some(include) = self.include {
            for x in include.into_iter() {
                params.push(("include", format!("{:?}", x)));
            }
        }

        params
    }
}

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
}

impl Client {
    pub fn new(api_key: String, api_shared_secret: String) -> Client {
        Client {
            url: String::from("https://letterboxd.com/api/v0/"),
            key: api_key,
            shared_secret: api_shared_secret,
        }
    }

    pub fn url(
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

    pub fn search(&self, search_request: SearchRequest) {
        let _ = self.url_for_search(search_request);
        // TODO: do search
    }

    fn url_for_search(&self, search_request: SearchRequest) -> String {
        self.url(hyper::Method::Get, "search", &search_request.into_url_params(), "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn get_test_client() -> Client {
        let key = String::from("4a168ac5ef7f124d03364db8be04394f319a4114a2e70695fa585ef778dd15e6");
        let secret =
            String::from("27be8dfc7d2c27e8cffb0b74a8e5c9235e70c71f6c34892677bd6746fbcc0b0b");
        Client::new(key, secret)
    }

    #[test]
    fn test_url() {
        let lbd = get_test_client();
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
    fn test_search() {
        let lbd = get_test_client();

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
