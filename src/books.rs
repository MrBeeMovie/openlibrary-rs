use std::fmt::Display;

use derive_builder::Builder;
use reqwest::Url;

use crate::OpenlibraryRequest;

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

/// The struct representation of a request to the [Books API](https://openlibrary.org/dev/docs/api/books)
///
/// The fields of this struct are private. If you want to view available fields that can be set please look at the [`BooksBuilder`] struct.
#[derive(Builder, Default, Debug)]
#[builder(setter(into), default)]
pub struct Books {
    book_type: BookType,
    id: String,
}

impl OpenlibraryRequest for Books {
    fn url(&self) -> Url {
        Url::parse(format!("{}/{}/{}.json", Self::host(), self.book_type, self.id).as_str())
            .unwrap()
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

        let _m = mock("GET", books.url().path())
            .with_header("content-type", "application/json")
            .with_body(json.to_string())
            .create();

        let books_result = books.execute();

        assert_eq!(books_result["title"], "test");
        assert_eq!(books_result["description"], "this is a test");
        assert_eq!(books_result["key"], "/works/1234");
    }
}
