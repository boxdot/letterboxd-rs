extern crate futures;
extern crate letterboxd;
extern crate tokio_core;

use std::env;
use std::process;

use futures::Future;
use tokio_core::reactor::Core;

fn usage_and_exit(_: env::VarError) -> String {
    println!(
        r#"This binary assumes that the following environment variables are set:
  API_KEY     letterboxd api key
  API_SECRET  letterboxd api secret
  USERNAME    letterboxd username
  PASSWORD    letterboxd password
"#
    );
    process::exit(1);
}

#[test]
#[ignore]
fn search() {
    let api_key = env::var("API_KEY").unwrap_or_else(usage_and_exit);
    let api_secret = env::var("API_SECRET").unwrap_or_else(usage_and_exit);

    let mut core = Core::new().unwrap();
    let client = letterboxd::Client::new(&core.handle(), api_key, api_secret);

    let mut req = letterboxd::SearchRequest::new(String::from("Fight Club"));
    req.per_page = Some(1);
    let do_search = client.search(&req, None);

    let do_check = |resp: letterboxd::SearchResponse| {
        let item = &resp.items[0];
        if let &letterboxd::AbstractSearchItem::FilmSearchItem { ref film, .. } = item {
            assert_eq!(film.name, "Fight Club");
        } else {
            assert!(false);
        }
        Ok(())
    };

    core.run(do_search.and_then(do_check)).unwrap();
}

#[test]
fn list() {
    // let api_key = env::var("API_KEY").unwrap_or_else(usage_and_exit);
    // let api_secret = env::var("API_SECRET").unwrap_or_else(usage_and_exit);
    // let username = env::var("USERNAME").unwrap_or_else(usage_and_exit);
    // let password = env::var("PASSWORD").unwrap_or_else(usage_and_exit);

    // let mut core = Core::new().unwrap();
    // let client = letterboxd::Client::new(&core.handle(), api_key, api_secret);

    // TODO:

    // 1. create a new list
    // 2. search for the list
    // 3. patch the list
    // 4. delete the list

    // let do_patch = |token| {
    //     let mut req = letterboxd::ListUpdateRequest::new(String::from("testing_list"));
    //     req.entries = vec![
    //             letterboxd::ListUpdateEntry::new(String::from("2a9q")),  // Fight Club
    //             letterboxd::ListUpdateEntry::new(String::from("bPI")),   // Melancholia
    //         ];

    //     // Letterboxd list id `LID` can be found on the share button
    //     client.patch_list(&lid, &req, &token)
    // };

    // let do_print = |resp| {
    //     print!("Patched {:?}", resp);
    //     Ok(())
    // };

    // let do_auth = client.auth(&username, &password);
    // core.run(do_auth.and_then(do_patch).and_then(do_print))
    //     .unwrap();
}
