use tokio::runtime::current_thread::Runtime;

const USAGE: &'static str = r#"This binary assumes that the following environment variables are set:
  LETTERBOXD_API_KEY       letterboxd api key
  LETTERBOXD_API_SECRET    letterboxd api secret
"#;

#[test]
#[ignore]
fn films() -> letterboxd::Result<()> {
    let api_key_pair = letterboxd::ApiKeyPair::from_env().expect(USAGE);
    let client = letterboxd::Client::new(api_key_pair);

    let req = letterboxd::FilmsRequest {
        per_page: Some(1),
        ..Default::default()
    };
    let resp = client.films(&req);

    let mut rt = Runtime::new().expect("valid runtime");
    let resp = rt.block_on(resp)?;
    println!("{:?}", resp);

    Ok(())
}

#[test]
#[ignore]
fn film_services() -> letterboxd::Result<()> {
    let api_key_pair = letterboxd::ApiKeyPair::from_env().expect(USAGE);
    let client = letterboxd::Client::new(api_key_pair);

    let mut rt = Runtime::new().expect("valid runtime");
    let resp = rt.block_on(client.film_services())?;
    println!("{:?}", resp);

    Ok(())
}

#[test]
#[ignore]
fn film_genres() -> letterboxd::Result<()> {
    let api_key_pair = letterboxd::ApiKeyPair::from_env().expect(USAGE);
    let client = letterboxd::Client::new(api_key_pair);

    let mut rt = Runtime::new().expect("valid runtime");
    let resp = rt.block_on(client.film_genres())?;
    println!("{:?}", resp);

    Ok(())
}

#[test]
#[ignore]
fn film() -> letterboxd::Result<()> {
    let api_key_pair = letterboxd::ApiKeyPair::from_env().expect(USAGE);
    let client = letterboxd::Client::new(api_key_pair);

    let resp = client.film("2a9q"); // Fight Club

    let mut rt = Runtime::new().expect("valid runtime");
    let film = rt.block_on(resp)?;
    println!("{:?}", film);
    assert_eq!(film.name, "Fight Club");

    Ok(())
}

#[test]
#[ignore]
fn film_availability() -> letterboxd::Result<()> {
    let api_key_pair = letterboxd::ApiKeyPair::from_env().expect(USAGE);
    let client = letterboxd::Client::new(api_key_pair);

    let resp = client.film_availability("2a9q"); // Fight Club

    let mut rt = Runtime::new().expect("valid runtime");
    let resp = rt.block_on(resp)?;
    println!("{:?}", resp);

    Ok(())
}

#[test]
#[ignore]
fn film_statistics() -> letterboxd::Result<()> {
    let api_key_pair = letterboxd::ApiKeyPair::from_env().expect(USAGE);
    let client = letterboxd::Client::new(api_key_pair);

    let resp = client.film_statistics("2a9q"); // Fight Club

    let mut rt = Runtime::new().expect("valid runtime");
    let resp = rt.block_on(resp)?;
    println!("{:?}", resp);

    Ok(())
}

#[test]
#[ignore]
fn list() -> letterboxd::Result<()> {
    let api_key_pair = letterboxd::ApiKeyPair::from_env().expect(USAGE);
    let client = letterboxd::Client::new(api_key_pair);

    let resp = client.list("1fKte"); // Collection

    let mut rt = Runtime::new().expect("valid runtime");
    let list = rt.block_on(resp)?;
    println!("{:?}", list);
    assert_eq!(list.name, "Collection");

    Ok(())
}

#[test]
#[ignore]
fn list_entries() -> letterboxd::Result<()> {
    let api_key_pair = letterboxd::ApiKeyPair::from_env().expect(USAGE);
    let client = letterboxd::Client::new(api_key_pair);

    let req = letterboxd::ListEntriesRequest::default();
    let resp = client.list_entries("1fKte", &req); // Collection

    let mut rt = Runtime::new().expect("valid runtime");
    let resp = rt.block_on(resp)?;
    println!("{:?}", resp);

    Ok(())
}

#[test]
#[ignore]
fn search() -> letterboxd::Result<()> {
    let api_key_pair = letterboxd::ApiKeyPair::from_env().expect(USAGE);
    let client = letterboxd::Client::new(api_key_pair);

    let req = letterboxd::SearchRequest {
        input: String::from("Fight Club"),
        per_page: Some(1),
        ..Default::default()
    };
    let resp = client.search(&req);

    let mut rt = Runtime::new().expect("valid runtime");
    let resp = rt.block_on(resp)?;
    println!("{:?}", resp);

    assert!(!resp.items.is_empty());
    let item = &resp.items[0];
    if let letterboxd::AbstractSearchItem::FilmSearchItem { ref film, .. } = *item {
        assert_eq!(film.name, "Fight Club");
    } else {
        panic!("found unexpected item: {:?}", item);
    }

    Ok(())
}
