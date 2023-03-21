//! [![github]](https://github.com/Tanja-4732/ocapi)&ensp;[![crates-io]](https://crates.io/crates/ocapi)&ensp;[![docs-rs]](https://docs.rs/ocapi/latest/ocapi)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! This is an unofficial client library for the [Opencast REST API](https://stable.opencast.org/rest_docs.html), written in [Rust](https://www.rust-lang.org/).
//!
//! This project is a work in progress, and is mainly focused on fetching/crawling the data required for the download of media packages.
//!
//! Notice: This project is not affiliated with the Opencast project in any way, shape, or form.

// Always included the data types
pub mod core;

// Include the client if the "client" feature is enabled (default)
#[cfg(feature = "client")]
pub mod client;
