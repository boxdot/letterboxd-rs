extern crate futures;
extern crate letterboxd;
extern crate tokio_core;

use std::env;
use std::process;

use futures::Future;
use tokio_core::reactor::Core;

fn exit_with_usage() -> String {
    println!("Usage: example <api_key> <api_secret> <username> <password> <listid>");
    process::exit(1);
}

fn main() {
    let mut args = env::args();
    args.next();
    let api_key = args.next().unwrap_or_else(exit_with_usage);
    let api_secret = args.next().unwrap_or_else(exit_with_usage);
    let username = args.next().unwrap_or_else(exit_with_usage);
    let password = args.next().unwrap_or_else(exit_with_usage);
    let lid = args.next().unwrap_or_else(exit_with_usage);

    let mut core = Core::new().unwrap();
    let client = letterboxd::Client::new(&core.handle(), api_key, api_secret);

    let do_patch = |token| {
        let mut req = letterboxd::ListUpdateRequest::new(String::from("testing_list"));
        req.entries = vec![
                letterboxd::ListUpdateEntry::new(String::from("2a9q")),  // Fight Club
                letterboxd::ListUpdateEntry::new(String::from("bPI")),   // Melancholia
            ];

        // Letterboxd list id `LID` can be found on the share button
        client.patch_list(&lid, &req, &token)
    };

    let do_print = |resp| {
        print!("Patched {:?}", resp);
        Ok(())
    };

    let do_auth = client.auth(&username, &password);
    core.run(do_auth.and_then(do_patch).and_then(do_print))
        .unwrap();
}
