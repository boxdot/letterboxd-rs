fn init() -> letterboxd::Client {
    dotenv::dotenv().ok();
    let api_key_pair = letterboxd::ApiKeyPair::from_env().expect("missing API key/secret env var");
    letterboxd::Client::new(api_key_pair)
}

#[ignore]
#[tokio::test]
async fn films() -> letterboxd::Result<()> {
    let client = init();
    let req = letterboxd::FilmsRequest {
        per_page: Some(1),
        ..Default::default()
    };
    let resp = client.films(&req).await?;
    println!("{:?}", resp);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn film_services() -> letterboxd::Result<()> {
    let client = init();
    let resp = client.film_services().await?;
    println!("{:?}", resp);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn film_genres() -> letterboxd::Result<()> {
    let client = init();
    let resp = client.film_genres().await?;
    println!("{:?}", resp);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn film() -> letterboxd::Result<()> {
    let client = init();
    let resp = client.film("2a9q").await?; // Fight Club
    println!("{:?}", resp);
    assert_eq!(resp.name, "Fight Club");

    Ok(())
}

#[ignore]
#[tokio::test]
async fn film_availability() -> letterboxd::Result<()> {
    let client = init();
    let resp = client.film_availability("2a9q").await?; // Fight Club
    println!("{:?}", resp);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn film_statistics() -> letterboxd::Result<()> {
    let client = init();
    let resp = client.film_statistics("2a9q").await?; // Fight Club
    println!("{:?}", resp);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn list() -> letterboxd::Result<()> {
    let client = init();
    let resp = client.list("1fKte").await?; // Collection
    println!("{:?}", resp);
    assert_eq!(resp.name, "Collection");

    Ok(())
}

#[ignore]
#[tokio::test]
async fn list_entries() -> letterboxd::Result<()> {
    let client = init();
    let req = letterboxd::ListEntriesRequest::default();
    let resp = client.list_entries("1fKte", &req).await?; // Collection
    println!("{:?}", resp);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn search() -> letterboxd::Result<()> {
    let client = init();
    let req = letterboxd::SearchRequest {
        input: String::from("Fight Club"),
        per_page: Some(1),
        ..Default::default()
    };
    let resp = client.search(&req).await?;
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

#[ignore]
#[tokio::test]
async fn films_include_genre() -> letterboxd::Result<()> {
    let client = init();
    let req = letterboxd::FilmsRequest {
        per_page: Some(1),
        // ai is LID for documentary genre
        include_genre: Some(vec!["ai".into()]),
        ..Default::default()
    };
    let resp = client.films(&req).await?;
    println!("{:?}", resp);

    Ok(())
}

#[tokio::test]
async fn films_exclude_genre() -> letterboxd::Result<()> {
    let client = init();
    let req = letterboxd::FilmsRequest {
        per_page: Some(1),
        // 7S is LID for drama genre, 7I = comedy, aC = horror
        exclude_genre: Some(vec!["7S".into(), "7I".into(), "aC".into()]),
        ..Default::default()
    };
    let resp = client.films(&req).await?;
    println!("{:?}", resp);

    Ok(())
}
