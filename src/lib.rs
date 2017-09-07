/// Api Docs: http://letterboxd-api.dev.cactuslab.com

extern crate crypto;
extern crate hex;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_url_params;
extern crate uuid;
extern crate futures;
extern crate tokio_core;

#[macro_use]
mod restmacro;
mod defs;
mod error;
mod helper;
mod rest;

pub use self::error::Error;
pub use defs::*;
pub use rest::Client;
