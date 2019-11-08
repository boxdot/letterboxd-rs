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
fn film_relationship() {
    let api_key = env::var("API_KEY").unwrap_or_else(usage_and_exit);
    let api_secret = env::var("API_SECRET").unwrap_or_else(usage_and_exit);
    let username = env::var("LB_USERNAME").unwrap_or_else(usage_and_exit);
    let password = env::var("LB_PASSWORD").unwrap_or_else(usage_and_exit);

    let client = letterboxd::Client::new(api_key, api_secret);

    let mut core = Core::new().unwrap();

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
fn list() {
    let api_key = env::var("API_KEY").unwrap_or_else(usage_and_exit);
    let api_secret = env::var("API_SECRET").unwrap_or_else(usage_and_exit);
    let username = env::var("LB_USERNAME").unwrap_or_else(usage_and_exit);
    let password = env::var("LB_PASSWORD").unwrap_or_else(usage_and_exit);

    let client = letterboxd::Client::new(api_key, api_secret);

    let mut core = Core::new().unwrap();

    let get_token = client.auth(&username, &password);
    let token = core.run(get_token).unwrap();

    // 1. create a new list
    // 2. search for the list
    // 3. patch the list
    // 4. delete the list

    const LIST_NAME: &'static str = "some_new_list";

    let do_create = client.post_list(
        &letterboxd::ListCreationRequest::new(String::from(LIST_NAME)),
        &token,
    );
    let do_find = |resp: letterboxd::ListCreateResponse| client.list(&resp.data.id, Some(&token));
    let do_patch = |list: letterboxd::List| {
        let mut req = letterboxd::ListUpdateRequest::new(String::from(LIST_NAME));
        req.entries = vec![
            letterboxd::ListUpdateEntry::new(String::from("2a9q")), // Fight Club
            letterboxd::ListUpdateEntry::new(String::from("bPI")),  /* Melancholia */
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
    )
    .unwrap();
}
