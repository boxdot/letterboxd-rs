extern crate futures;
extern crate letterboxd;
extern crate tokio_core;

use futures::Future;
use tokio_core::reactor::Core;

const LETTERBOXD_API_KEY: &'static str = "your_key";
const LETTERBOXD_API_SECRET: &'static str = "your_secret";
const USERNAME: &'static str = "your_username";
const PASSWORD: &'static str = "your_password";

fn main() {
    let mut core = Core::new().unwrap();
    let key = String::from(LETTERBOXD_API_KEY);
    let secret = String::from(LETTERBOXD_API_SECRET);
    let client = letterboxd::Client::new(&core.handle(), key, secret);

    let do_patch = |token| {
        let mut req = letterboxd::ListUpdateRequest::new(String::from("testing_list"));
        req.entries = vec![
                letterboxd::ListUpdateEntry::new(String::from("2a9q")),  // Fight Club
                letterboxd::ListUpdateEntry::new(String::from("bPI")),   // Melancholia
            ];

        // Letterboxd list id `LID` can be found on the share button
        client.patch_list("LID", &req, &token)
    };

    let do_print = |resp| {
        print!("Patched {:?}", resp);
        Ok(())
    };

    let do_auth = client.auth(USERNAME, PASSWORD);
    core.run(do_auth.and_then(do_patch).and_then(do_print))
        .unwrap();
}
