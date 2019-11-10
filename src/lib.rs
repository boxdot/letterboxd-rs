//! This module wraps the Letterboxd API which provides easy and flexible
//! access to data on the Letterboxd.com website.
//!
//! [Client](struct.Client.html)'s API follows the following rules:
//!
//! * All Letterboxd API calls are asynchronous.
//! * A client is always created from API key and secret. If auth token,
//!   is provided, client calls will be authenticated. Client can be
//!   created from username/password. A token can be set after client was created.
//! * API key and secret can be created from default environment variables.
//! * Except GET calls all methods include a path parameter.
//!
//! Further, most of [Client](struct.Client.html)'s methods take a request
//! struct, which is then serialized to url encoded parameters and return a
//! response type, which is deserialized from JSON. However, some methods omit
//! the request or/and the response struct.
//!
//! Entities are identified in the API by Letterboxd ID (or LID), an
//! alpha-numeric string value that is returned where appropriate. For films,
//! lists and reviews, the LID can also be found through the Letterboxd website
//! as the path portion of the entity’s shareable boxd.it link.
//!
//! For more information, cf. API docs at
//! http://api-docs.letterboxd.com
//!
//! # Examples
//!
//! Client without authentication:
//!
//! ```rust,no_run
//! use tokio::runtime::current_thread::Runtime;
//!
//! let api_key_pair = letterboxd::ApiKeyPair::from_env().expect("missing api key/secret");
//! let client = letterboxd::Client::new(api_key_pair);
//!
//! let req = letterboxd::FilmsRequest {
//!     per_page: Some(1),
//!     ..Default::default()
//! };
//! let resp = client.films(&req);
//!
//! let mut rt = Runtime::new().expect("valid runtime");
//! let resp = rt.block_on(resp).expect("request failed");
//! println!("{:?}", resp);
//! ```
//!
//! Create and authenticate client from username/password;
//!
//! ```rust,no_run
//! use tokio::runtime::current_thread::Runtime;
//!
//! let api_key_pair = letterboxd::ApiKeyPair::from_env().expect("missing api key/secret");
//! let username = std::env::var("LETTERBOXD_USERNAME").expect("missing username");
//! let password = std::env::var("LETTERBOXD_PASSWORD").expect("missing password");
//!
//! let res = async {
//!     let client = letterboxd::Client::authenticate(api_key_pair, &username, &password).await?;
//!
//!     let req = letterboxd::FilmRelationshipUpdateRequest {
//!         watched: Some(true),
//!         ..Default::default()
//!     };
//!     client.update_film_relationship("2a9q", &req).await?; // Fight Club
//!
//!     Ok::<_, letterboxd::Error>(())
//! };
//!
//! let mut rt = Runtime::new().expect("valid runtime");
//! let resp = rt.block_on(res).expect("request failed");
//! ```

mod client;
mod defs;
mod error;
mod helper;

pub use client::{ApiKeyPair, Client};
pub use defs::*;
pub use error::{Error, Result};
