extern crate futures;
extern crate letterboxd;
extern crate tokio_core;

use std::env;
use std::process;

use futures::Future;
use tokio_core::reactor::Core;

fn exit_with_usage() -> String {
    println!("Usage: example <api_key> <api_secret> <input>");
    process::exit(1);
}

fn main() {
    let mut args = env::args();
    args.next();
    let api_key = args.next().unwrap_or_else(exit_with_usage);
    let api_secret = args.next().unwrap_or_else(exit_with_usage);
    let input = args.next().unwrap_or_else(exit_with_usage);

    let mut core = Core::new().unwrap();
    let client = letterboxd::Client::new(&core.handle(), api_key, api_secret);

    let req = letterboxd::SearchRequest::new(input);
    let do_search = client.search(&req, None);

    let do_print = |resp| {
        print!("Response {:?}", resp);
        Ok(())
    };

    core.run(do_search.and_then(do_print)).unwrap();
}
