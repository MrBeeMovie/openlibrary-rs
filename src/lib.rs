//! Wrapper for [Openlibrary's Web API](https://openlibrary.org/developers/api)
//!
//! There are currently 8 sub APIs offered by Openlibrary's Web API.
//! This library is still in development and does not have a wrapper for each yet.
//! You can check the table below to see the status of each.
//!
//! - [ ] Books
//! - [ ] Authors
//! - [ ] Subjects
//! - [X] Search
//! - [ ] Search inside
//! - [ ] Partner
//! - [ ] Covers
//! - [ ] Recent Changes
//!
//! # Search
//!
//! You can search for books, authors, and more using the [`search::Search`] struct[^note]
//!
//! Example
//! ``` rust
//! use openlibrary_rs::search::SearchBuilder;
//!
//! // execute search and pretty print debug of first result
//! let results = SearchBuilder::default()
//!     .query("the lord of the rings")
//!     .build()
//!     .unwrap();
//!
//! println!("{:#?}", results.execute().docs[0]);
//! ```
//!
//! [^note]: You must use the [`search::SearchBuilder`] to build instances of [`search::Search`] as all fields are private

pub mod search;
