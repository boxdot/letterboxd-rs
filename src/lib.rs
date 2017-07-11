extern crate crypto;
extern crate hex;
extern crate uuid;
extern crate hyper;

use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;
use hex::ToHex;
use uuid::Uuid;

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

pub struct LetterboxdAPI {
    url: String,
    key: String,
    shared_secret: String,
}

impl LetterboxdAPI {
    pub fn new(api_key: String, api_shared_secret: String) -> LetterboxdAPI {
        LetterboxdAPI {
            url: String::from("https://letterboxd.com/api/v0/"),
            key: api_key,
            shared_secret: api_shared_secret,
        }
    }

    pub fn url(
        &self,
        method: hyper::Method,
        endpoint: &str,
        parameters: &[(String, String)],
        body: &str,
    ) -> String {
        self.url_with_nonce_and_timestamp(method, endpoint, parameters, body, nonce(), now())
    }

    fn url_with_nonce_and_timestamp(
        &self,
        method: hyper::Method,
        endpoint: &str,
        parameters: &[(String, String)],
        body: &str,
        nonce: Uuid,
        timestamp: u64,
    ) -> String {
        let parameters: Vec<String> = parameters
            .iter()
            .map(|&(ref key, ref val)| format!("{}={}", key, val))
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_url() {
        let key = String::from("4a168ac5ef7f124d03364db8be04394f319a4114a2e70695fa585ef778dd15e6");
        let secret =
            String::from("27be8dfc7d2c27e8cffb0b74a8e5c9235e70c71f6c34892677bd6746fbcc0b0b");
        let lbd = LetterboxdAPI::new(key, secret);

        let uuid = Uuid::from_str("9d54386f-118e-4876-b8e8-92ba37d451e7")
            .expect("Uuid::from_str failed to parse example uuid.");
        let timestamp = 1499803866u64;
        assert_eq!(
            lbd.url_with_nonce_and_timestamp(
                hyper::Method::Get,
                "film/2a9q",
                &vec![(String::from("foo"), String::from("bar"))],
                "",
                uuid,
                timestamp,
            ),
            "https://letterboxd.com/api/v0/film/2a9q?apikey=4a168ac5ef7f124d03364db8be04394f319a4114a2e70695fa585ef778dd15e6&nonce=9d54386f-118e-4876-b8e8-92ba37d451e7&timestamp=1499803866&foo=bar&signature=46fe62e84e3b3d417cb539a9d3a5ea79f51f37cc5311d4583ef7d1f9444f8797"
        );
    }
}
