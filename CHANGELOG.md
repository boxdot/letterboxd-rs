# Changelog

## [v0.3.0](https://github.com/boxdot/letterboxd-rs/compare/v0.2.2...v0.3.0)

### Changed

* Upgraded hyper to 0.14 (and therefore implicitly to tokio 1.0). [#19](https://github.com/boxdot/letterboxd-rs/pull/19)
* `letterboxd::Error` is `Send` and `Sync` [#21](https://github.com/boxdot/letterboxd-rs/pull/21)
