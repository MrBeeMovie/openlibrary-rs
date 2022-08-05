use std::{collections::HashMap, fmt::Display};

use derive_builder::Builder;
use serde_json::Value;

use crate::OpenlibraryRequest;

pub struct BooksResult {
    pub fields: Vec<HashMap<String, Value>>,
}

#[derive(Default, Clone, Debug)]
pub enum BookType {
    #[default]
    Works,
    Editions,
    ISBN,
}

impl Display for BookType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Works => "/works",
                Self::Editions => "/books",
                Self::ISBN => "/isbn",
            }
        )
    }
}

#[derive(Builder, Default, Debug)]
#[builder(setter(into), default)]
pub struct Books {
    pub(super) book_type: BookType,
    pub(super) id: String,
}

impl Books {
    pub fn execute(&self) -> HashMap<String, Value> {
        let request = OpenlibraryRequest::books_request(self);
        let response = request.execute().unwrap();

        response.json().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use mockito::mock;
    use serde_json::json;

    use crate::OpenlibraryRequest;

    use super::BooksBuilder;

    #[test]
    fn test_books_execute_valid_response() {
        let books = BooksBuilder::default().id("1234").build().unwrap();

        let json = json!({
            "title": "test",
            "description": "this is a test",
            "key": "/works/1234"
        });

        let request = OpenlibraryRequest::books_request(&books);
        let endpoint = &request.url[request.url.find("/works").unwrap()..];

        let _m = mock("GET", endpoint)
            .with_header("content-type", "application/json")
            .with_body(json.to_string())
            .create();

        let books_result = books.execute();

        assert_eq!(books_result.get("title").unwrap(), "test");
        assert_eq!(books_result.get("description").unwrap(), "this is a test");
        assert_eq!(books_result.get("key").unwrap(), "/works/1234");
    }
}
