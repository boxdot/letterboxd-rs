# Letterboxd API for Rust ![rust build status](https://github.com/boxdot/letterboxd-rs/workflows/rust/badge.svg)

[Letterboxd API](http://api-docs.letterboxd.com) client for accessing the data on the
Letterboxd.com website in Rust.

## Example

```rust
use tokio::runtime::Runtime;

let api_key_pair = letterboxd::ApiKeyPair::from_env().unwrap();
let client = letterboxd::Client::new(api_key_pair);

let req = letterboxd::SearchRequest {
    input: "Fight Club".to_string(),
    per_page: Some(1),
    ..Default::default()
};
let resp = client.search(&req);

let mut rt = Runtime::new().unwrap();
let resp = rt.block_on(resp).unwrap();
println!("{:?}", resp);
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
