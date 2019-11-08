use std::time;

use crypto::hmac;
use crypto::mac::Mac;
use crypto::sha2;
use hex::ToHex;
use uuid;

pub fn nonce() -> uuid::Uuid {
    uuid::Uuid::new_v4()
}

pub fn now() -> u64 {
    let now = time::SystemTime::now();
    let dur = now
        .duration_since(time::UNIX_EPOCH)
        .expect("SystemTime::duration_since failed");
    dur.as_secs()
}

pub fn hmac_sha256(secret: &str, msg: &str) -> String {
    let mut hmac = hmac::Hmac::new(sha2::Sha256::new(), secret.as_bytes());
    hmac.input(msg.as_bytes());
    hmac.result().code().encode_hex()
}
