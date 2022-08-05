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
//! - [X] [Books](https://openlibrary.org/dev/docs/api/books) [^books_completeness]
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
//! # Search
//!
//! You can search for books, authors, and more using the [`search::Search`] struct
//!
//! ``` rust
//! use openlibrary_rs::search::SearchBuilder;
//!
//! // execute search and pretty print debug of first result
//! let search = SearchBuilder::default()
//!     .query("the lord of the rings")
//!     .build()
//!     .unwrap();
//!
//! println!("{:#?}", search.execute().docs[0]);
//! ```
//!
//! [^books_completeness]: Everything excluding the generic Books API is complete(i.e. Works, Editions, and ISBN APIs are done).
//!

#[allow(dead_code)]
const OPENLIBRARY_URL: &str = "https://openlibrary.org";

pub mod books;
pub mod search;

use reqwest::blocking::Response as ReqwestResponse;
use reqwest::Result as ReqwestResult;

use crate::books::Books;
use crate::search::Search;

struct OpenlibraryRequest {
    url: String,
}

impl OpenlibraryRequest {
    fn root_url() -> String {
        #[cfg(not(test))]
        return OPENLIBRARY_URL.to_string();
        #[cfg(test)]
        return mockito::server_url().to_string();
    }

    pub fn search_request(search: &Search) -> OpenlibraryRequest {
        let mut url = Self::root_url();

        url.push_str("/search");
        url.push_str(search.search_type.to_string().as_str());

        url.push_str(format!(".json?page={}&limit={}", search.page, search.limit,).as_str());

        if let Some(query) = search.query.as_deref() {
            url.push_str(format!("&q={}", query).as_str())
        }

        if !search.fields.is_empty() {
            url.push_str(format!("&fields={}", search.fields.join(",")).as_str())
        }

        OpenlibraryRequest { url }
    }

    pub fn books_request(books: &Books) -> OpenlibraryRequest {
        let mut url = Self::root_url();

        url.push_str(format!("{}/{}.json", books.book_type, books.id).as_str());

        OpenlibraryRequest { url }
    }

    pub fn execute(&self) -> ReqwestResult<ReqwestResponse> {
        reqwest::blocking::get(self.url.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::books::BooksBuilder;
    use crate::search::SearchBuilder;
    use crate::OpenlibraryRequest;

    #[test]
    pub fn test_search_request() {
        let search = SearchBuilder::default().build().unwrap();
        let search_request = OpenlibraryRequest::search_request(&search);

        assert_eq!(
            search_request.url,
            format!("{}/search.json?page=1&limit=10", mockito::server_url())
        )
    }

    #[test]
    pub fn test_books_request() {
        let books = BooksBuilder::default().id("1234").build().unwrap();
        let books_request = OpenlibraryRequest::books_request(&books);

        assert_eq!(
            books_request.url,
            format!("{}/works/1234.json", mockito::server_url())
        )
    }
}
