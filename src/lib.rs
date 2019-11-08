//! This module wraps the Letterboxd API which provides easy and flexible
//! access to data on the Letterboxd.com website.
//!
//! [Client](struct.Client.html)'s API follows the following rules:
//!
//! * All Letterboxd API calls are asynchronous.
//! * Each method takes an optional argument `token`, which enabled
//!   authentication for the call.
//! * Except for GET calls all methods include a path parameter.
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
//! # Example
//! ```rust,no_run
//! # extern crate futures;
//! # extern crate letterboxd;
//! # extern crate tokio_core;
//! #
//! # use futures::future::Future;
//! # use tokio_core::reactor::Core;
//! #
//! # const USERNAME: &'static str = "some key";
//! # const PASSWORD: &'static str = "some key";
//! #
//! # fn main() {
//! #   let api_key = String::from("some_key");
//! #   let api_secret = String::from("some_secret");
//! #
//! let client = letterboxd::Client::new(api_key, api_secret);
//!
//! let get_token = client.auth(&USERNAME, &PASSWORD);
//! let req = letterboxd::FilmRelationshipUpdateRequest {
//!     watched: Some(true),
//!     ..Default::default()
//! };
//! let do_update = |token| {
//!     client.update_film_relationship("2a9q", &req, &token) // Fight Club
//! };
//! // execute on some runtime, e.g. with:
//! let mut core = tokio_core::reactor::Core::new().unwrap();
//! core.run(get_token.and_then(do_update)).unwrap();
//! # }
//! ```
//!

extern crate crypto;
extern crate hex;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate serde_derive;
extern crate futures;
extern crate serde_json;
extern crate serde_url_params;
extern crate uuid;

#[macro_use]
mod rest;
mod client;
mod defs;
mod error;
mod helper;

pub use self::error::Error;
pub use client::Client;
pub use defs::*;
