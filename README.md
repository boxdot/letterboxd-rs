# Letterboxd API for Rust [![CircleCI](https://circleci.com/gh/boxdot/letterboxd-rs/tree/master.svg?style=shield)](https://circleci.com/gh/boxdot/letterboxd-rs/tree/master)

Letterboxd API for access to data on the Letterboxd.com website in Rust.

Note: The [Letterboxd API](http://letterboxd-api.dev.cactuslab.com) has a beta status right now.

## Example

```rust
let mut core = Core::new().unwrap();
let client = letterboxd::Client::new(&core.handle(), API_KEY, API_SECRET);

let mut req = letterboxd::SearchRequest::new(String::from("Fight Club"));
let do_search = client.search(&req, None /* no auth token needed */);

let do_print = |resp| {
    println!("{:?}", resp);
    Ok(())
};

core.run(do_search.and_then(do_print)).unwrap();
```

For more examples cf. `tests/integration.rs`.

## Progress

- [x] Request signing
- [ ] Endpoint Auth
- [ ] Endpoint Comment
- [ ] Endpoint Contributor
- [x] Endpoint Film (except for `film/report`)
- [x] Endpoint List
- [ ] Endpoint Log-Entry
- [ ] Endpoint Me
- [ ] Endpoint Member
- [x] Endpoint Search

## License

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this document by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
