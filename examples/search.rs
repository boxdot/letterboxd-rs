#[tokio::main]
async fn main() -> letterboxd::Result<()> {
    let api_key_pair = letterboxd::ApiKeyPair::from_env().unwrap();
    let client = letterboxd::Client::new(api_key_pair);

    let req = letterboxd::SearchRequest {
        input: "Fight Club".to_string(),
        per_page: Some(1),
        ..Default::default()
    };
    let resp = client.search(&req).await?;
    println!("{:?}", resp);

    Ok(())
}
