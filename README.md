# Letterboxd API for Rust

[![crates-badge]][crates-url]
[![docs-badge]][docs-url]
[![license-badge]][license]
[![CI](https://github.com/boxdot/letterboxd-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/boxdot/letterboxd-rs/actions/workflows/rust.yml)

[crates-badge]: https://img.shields.io/crates/v/letterboxd.svg
[crates-url]: https://crates.io/crates/letterboxd
[docs-badge]: https://docs.rs/letterboxd/badge.svg
[docs-url]: https://docs.rs/letterboxd
[license-badge]: https://img.shields.io/crates/l/letterboxd.svg
[license]: #license

[Letterboxd API](http://api-docs.letterboxd.com) client for accessing the data on the
Letterboxd.com website in Rust.

## Example

```rust
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
```

For more examples cf. `tests/integration.rs`.

*Note*: Not all APIs are implemented. Feel free to contribute missing implementation, usually these
are very straight forward.

## License

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this document by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
