use std::env;

use tokio::runtime::current_thread::Runtime;

const USAGE: &'static str =
    r#"This binary assumes that the following environment variables are set:
  LETTERBOXD_API_KEY       letterboxd api key
  LETTERBOXD_API_SECRET    letterboxd api secret
  LETTERBOXD_USERNAME      letterboxd user name
  LETTERBOXD_PASSWORD      letterboxd password
"#;

#[test]
#[ignore]
fn film_relationship() {
    let api_key_pair = letterboxd::ApiKeyPair::from_env().expect(USAGE);
    let username = env::var("LETTERBOXD_USERNAME").expect(USAGE);
    let password = env::var("LETTERBOXD_PASSWORD").expect(USAGE);

    let res = async {
        let client = letterboxd::Client::authenticate(api_key_pair, &username, &password).await?;

        const FIGHT_CLUB_ID: &str = "2a9q";

        let film_relationship = client.film_relationship(FIGHT_CLUB_ID).await?; // Fight Club
        println!("{:?}", film_relationship);

        let req = letterboxd::FilmRelationshipUpdateRequest {
            watched: Some(true),
            ..Default::default()
        };
        let res = client.update_film_relationship(FIGHT_CLUB_ID, &req).await?;
        println!("{:?}", res);

        let req = letterboxd::MemberFilmRelationshipsRequest {
            per_page: Some(1),
            ..Default::default()
        };
        let film_relationship_members = client
            .film_relationship_members(FIGHT_CLUB_ID, &req)
            .await?;
        println!("{:?}", film_relationship_members);

        Ok::<_, letterboxd::Error>(())
    };

    let mut rt = Runtime::new().expect("valid runtime");
    rt.block_on(res).expect("request failed");
}

#[test]
#[ignore]
fn list() {
    let api_key_pair = letterboxd::ApiKeyPair::from_env().expect(USAGE);
    let username = env::var("LETTERBOXD_USERNAME").expect(USAGE);
    let password = env::var("LETTERBOXD_PASSWORD").expect(USAGE);

    let res = async {
        let client = letterboxd::Client::authenticate(api_key_pair, &username, &password).await?;

        // 1. create a new list
        // 2. search for the list
        // 3. patch the list
        // 4. delete the list

        const LIST_NAME: &'static str = "new list";

        let resp = client
            .create_list(&letterboxd::ListCreationRequest::new(String::from(
                LIST_NAME,
            )))
            .await?;

        let list = client.list(&resp.data.id).await?;

        let req = letterboxd::ListUpdateRequest {
            entries: vec![
                letterboxd::ListUpdateEntry::new(String::from("2a9q")), // Fight Club
                letterboxd::ListUpdateEntry::new(String::from("bPI")),  // Melancholia
            ],
            ..letterboxd::ListUpdateRequest::new(String::from(LIST_NAME))
        };
        let resp = client.update_list(&list.id, &req).await?;
        assert_eq!(resp.data.name, LIST_NAME);
        assert_eq!(resp.data.film_count, 2);

        client.delete_list(&resp.data.id).await?;

        Ok::<_, letterboxd::Error>(())
    };

    let mut rt = Runtime::new().expect("valid runtime");
    rt.block_on(res).expect("request failed");
}
