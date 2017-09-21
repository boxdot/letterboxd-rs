extern crate futures;
extern crate letterboxd;
extern crate tokio_core;

use std::env;
use std::fmt;
use std::process;

use futures::Future;
use tokio_core::reactor::Core;

fn usage_and_exit(_: env::VarError) -> String {
    println!(
        r#"This binary assumes that the following environment variables are set:
  API_KEY       letterboxd api key
  API_SECRET    letterboxd api secret
"#
    );
    process::exit(1);
}

fn do_print<T>(resp: T) -> Result<T, letterboxd::Error>
where
    T: fmt::Debug,
{
    println!("{:?}", resp);
    Ok(resp)
}

#[test]
#[ignore]
fn films() {
    let api_key = env::var("API_KEY").unwrap_or_else(usage_and_exit);
    let api_secret = env::var("API_SECRET").unwrap_or_else(usage_and_exit);

    let mut core = Core::new().unwrap();
    let client = letterboxd::Client::new(&core.handle(), api_key, api_secret);

    let mut req = letterboxd::FilmsRequest::default();
    req.per_page = Some(1);
    let do_get_films = client.films(&req, None);

    core.run(do_get_films.and_then(do_print)).unwrap();
}

#[test]
#[ignore]
fn film_services() {
    let api_key = env::var("API_KEY").unwrap_or_else(usage_and_exit);
    let api_secret = env::var("API_SECRET").unwrap_or_else(usage_and_exit);

    let mut core = Core::new().unwrap();
    let client = letterboxd::Client::new(&core.handle(), api_key, api_secret);

    let do_get_film_services = client.film_services(None);

    core.run(do_get_film_services.and_then(do_print)).unwrap();
}

#[test]
#[ignore]
fn film_genres() {
    let api_key = env::var("API_KEY").unwrap_or_else(usage_and_exit);
    let api_secret = env::var("API_SECRET").unwrap_or_else(usage_and_exit);

    let mut core = Core::new().unwrap();
    let client = letterboxd::Client::new(&core.handle(), api_key, api_secret);

    let do_get_film_genres = client.film_genres(None);

    core.run(do_get_film_genres.and_then(do_print)).unwrap();
}

#[test]
#[ignore]
fn film() {
    let api_key = env::var("API_KEY").unwrap_or_else(usage_and_exit);
    let api_secret = env::var("API_SECRET").unwrap_or_else(usage_and_exit);

    let mut core = Core::new().unwrap();
    let client = letterboxd::Client::new(&core.handle(), api_key, api_secret);

    let do_get_film = client.film("2a9q", None); // Fight Club
    let do_check = |film: letterboxd::Film| {
        assert_eq!(film.name, "Fight Club");
        Ok(film)
    };

    core.run(do_get_film.and_then(do_check).and_then(do_print))
        .unwrap();
}

#[test]
#[ignore]
fn film_availability() {
    let api_key = env::var("API_KEY").unwrap_or_else(usage_and_exit);
    let api_secret = env::var("API_SECRET").unwrap_or_else(usage_and_exit);

    let mut core = Core::new().unwrap();
    let client = letterboxd::Client::new(&core.handle(), api_key, api_secret);

    let do_get_film_availability = client.film_availability("2a9q", None); // Fight Club

    core.run(do_get_film_availability.and_then(do_print))
        .unwrap();
}

#[test]
#[ignore]
fn film_statistics() {
    let api_key = env::var("API_KEY").unwrap_or_else(usage_and_exit);
    let api_secret = env::var("API_SECRET").unwrap_or_else(usage_and_exit);

    let mut core = Core::new().unwrap();
    let client = letterboxd::Client::new(&core.handle(), api_key, api_secret);

    let do_get_film_stats = client.film_statistics("2a9q", None); // Fight Club

    core.run(do_get_film_stats.and_then(do_print)).unwrap();
}

#[test]
#[ignore]
fn list() {
    let api_key = env::var("API_KEY").unwrap_or_else(usage_and_exit);
    let api_secret = env::var("API_SECRET").unwrap_or_else(usage_and_exit);

    let mut core = Core::new().unwrap();
    let client = letterboxd::Client::new(&core.handle(), api_key, api_secret);

    let do_req = client.list("1dgps", None); // testing_list
    let do_check = |list: letterboxd::List| {
        assert_eq!(list.name, "testing_list");
        Ok(list)
    };

    core.run(do_req.and_then(do_check).and_then(do_print))
        .unwrap();
}

#[test]
#[ignore]
fn list_entries() {
    let api_key = env::var("API_KEY").unwrap_or_else(usage_and_exit);
    let api_secret = env::var("API_SECRET").unwrap_or_else(usage_and_exit);

    let mut core = Core::new().unwrap();
    let client = letterboxd::Client::new(&core.handle(), api_key, api_secret);

    // 1dgps -> testing_list
    let do_req = client.list_entries("1dgps", &letterboxd::ListEntriesRequest::default(), None);
    core.run(do_req.and_then(do_print)).unwrap();
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
        if let letterboxd::AbstractSearchItem::FilmSearchItem { ref film, .. } = *item {
            assert_eq!(film.name, "Fight Club");
        } else {
            assert!(false);
        }
        Ok(())
    };

    core.run(do_search.and_then(do_check)).unwrap();
}
