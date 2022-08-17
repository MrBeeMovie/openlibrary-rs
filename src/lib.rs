//! Wrapper for [Openlibrary's Web API](https://openlibrary.org/developers/api)
//!
//! # Overview
//!
//! **PLEASE NOTE**: this crate is currently in an experimental stage.
//! Meaning expect frequent, large breaking changes from version to version until we are in a stable state.
//!
//! There are currently 8 sub APIs offered by Openlibrary's Web API.
//! You can check the table below to see the status of each.
//!
//! - [X] [Books](https://openlibrary.org/dev/docs/api/books)
//! - [ ] [Authors](https://openlibrary.org/dev/docs/api/authors)
//! - [ ] [Subjects](https://openlibrary.org/dev/docs/api/subjects)
//! - [X] [Search](https://openlibrary.org/dev/docs/api/search)
//! - [ ] [Search inside](https://openlibrary.org/dev/docs/api/search_inside)
//! - [ ] [Partner](https://openlibrary.org/dev/docs/api/read)
//! - [ ] [Covers](https://openlibrary.org/dev/docs/api/covers)
//! - [ ] [Recent Changes](https://openlibrary.org/dev/docs/api/recentchanges)
//!
//! # Books
//!
//! You can view information about books by Works, Editions, and ISBN ids by using the [`books::Books`] struct
//!
//! ``` rust
//! use openlibrary_rs::books::{BookType, BooksBuilder};
//! use openlibrary_rs::OpenlibraryRequest;
//!
//! // execute request to Works API and pretty print debug of result
//! let books = BooksBuilder::default()
//!     .book_type(BookType::Works)
//!     .id("OL45883W")
//!     .build()
//!     .unwrap();
//!
//!     println!("{:#?}", books.execute());
//! ```
//!
//! You can view information about multiple books by using the [`books::BooksGeneric`] struct
//!
//! ``` rust
//! use openlibrary_rs::books::{BookType, BooksGenericBuilder};
//! use openlibrary_rs::OpenlibraryRequest;
//!
//! let books_generic = BooksGenericBuilder::default()
//!     .bibkeys(vec![
//!         "ISBN:0201558025".to_string(),
//!         "LCCN:93005405".to_string(),
//!     ])
//!     .build()
//!     .unwrap();
//!
//! println!("{:#?}", books_generic.execute());
//! ```
//!
//! # Search
//!
//! You can search for books, authors, and more using the [`search::Search`] struct
//!
//! ``` rust
//! use openlibrary_rs::search::SearchBuilder;
//! use openlibrary_rs::OpenlibraryRequest;
//!
//! // execute search and pretty print debug of first result
//! let search = SearchBuilder::default()
//!     .query("the lord of the rings")
//!     .build()
//!     .unwrap();
//!
//! println!("{:#?}", search.execute());
//! ```
//!
use reqwest::Url;
use serde_json::Value;

#[allow(dead_code)]
const OPENLIBRARY_HOST: &str = "https://openlibrary.org";

pub mod books;
pub mod search;

/// Trait representation of an Openlibrary request
pub trait OpenlibraryRequest {
    fn host() -> String {
        #[cfg(not(test))]
        return OPENLIBRARY_HOST.to_string();
        #[cfg(test)]
        return mockito::server_url().to_string();
    }

    fn path(&self) -> String;

    fn query(&self) -> Vec<(&'static str, String)>;

    fn url(&self) -> Url {
        Url::parse_with_params(
            format!("{}{}", Self::host(), self.path()).as_str(),
            self.query(),
        )
        .unwrap()
    }

    fn execute(&self) -> Value {
        let response = reqwest::blocking::get(self.url()).unwrap();
        response.json().unwrap()
    }
}
