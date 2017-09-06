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
