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
  LB_USERNAME   letterboxd username
  LB_PASSWORD   letterboxd password
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
fn film_relationship() {
    let api_key = env::var("API_KEY").unwrap_or_else(usage_and_exit);
    let api_secret = env::var("API_SECRET").unwrap_or_else(usage_and_exit);
    let username = env::var("LB_USERNAME").unwrap_or_else(usage_and_exit);
    let password = env::var("LB_PASSWORD").unwrap_or_else(usage_and_exit);

    let mut core = Core::new().unwrap();
    let client = letterboxd::Client::new(&core.handle(), api_key, api_secret);

    let get_token = client.auth(&username, &password);
    let token = core.run(get_token).unwrap();

    let do_get_film_relationship = client.film_relationship("2a9q", Some(&token)); // Fight Club
    core.run(do_get_film_relationship.and_then(do_print))
        .unwrap();

    let mut req = letterboxd::FilmRelationshipUpdateRequest::default();
    req.watched = Some(true);
    let do_update_film_relationship = client.update_film_relationship("2a9q", &req, &token);
    core.run(do_update_film_relationship.and_then(do_print))
        .unwrap();

    let mut req = letterboxd::MemberFilmRelationshipsRequest::default();
    req.per_page = Some(1);
    let do_get_film_relationship_members =
        client.film_relationship_members("2a9q", &req, Some(&token));
    core.run(do_get_film_relationship_members.and_then(do_print))
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

#[test]
#[ignore]
fn list() {
    let api_key = env::var("API_KEY").unwrap_or_else(usage_and_exit);
    let api_secret = env::var("API_SECRET").unwrap_or_else(usage_and_exit);
    let username = env::var("LB_USERNAME").unwrap_or_else(usage_and_exit);
    let password = env::var("LB_PASSWORD").unwrap_or_else(usage_and_exit);

    let mut core = Core::new().unwrap();
    let client = letterboxd::Client::new(&core.handle(), api_key, api_secret);

    let get_token = client.auth(&username, &password);
    let token = core.run(get_token).unwrap();

    // 1. create a new list
    // 2. search for the list
    // 3. patch the list
    // 4. delete the list

    const LIST_NAME: &'static str = "some_new_list";

    let do_create =
        client.post_list(&letterboxd::ListCreationRequest::new(String::from(LIST_NAME)), &token);
    let do_find = |resp: letterboxd::ListCreateResponse| client.list(&resp.data.id, Some(&token));
    let do_patch = |list: letterboxd::List| {
        let mut req = letterboxd::ListUpdateRequest::new(String::from(LIST_NAME));
        req.entries = vec![
                letterboxd::ListUpdateEntry::new(String::from("2a9q")),  // Fight Club
                letterboxd::ListUpdateEntry::new(String::from("bPI")),   // Melancholia
            ];
        client.patch_list(&list.id, &req, &token)
    };
    let check_patch = |resp: letterboxd::ListUpdateResponse| {
        assert_eq!(resp.data.name, LIST_NAME);
        assert_eq!(resp.data.film_count, 2);
        Ok(resp)
    };
    let do_delete =
        |resp: letterboxd::ListUpdateResponse| client.delete_list(&resp.data.id, &token);

    core.run(
        do_create
            .and_then(do_print)
            .and_then(do_find)
            .and_then(do_print)
            .and_then(do_patch)
            .and_then(do_print)
            .and_then(check_patch)
            .and_then(do_delete),
    ).unwrap();
}
