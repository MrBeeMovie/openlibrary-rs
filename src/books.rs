use std::fmt::Display;

use derive_builder::Builder;

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
    fn path(&self) -> String {
        format!("{}/{}.json", self.book_type, self.id)
    }

    fn query(&self) -> Vec<(&'static str, String)> {
        vec![]
    }
}

/// The struct representation of a request to the generic [Books API](https://openlibrary.org/dev/docs/api/books)
///
/// The fields of this struct are private. If you want to view available fields that can be set please look at the [`BooksGenericBuilder`] struct.
#[derive(Builder, Default, Debug)]
#[builder(setter(into), default)]
pub struct BooksGeneric {
    #[builder(default = "vec![]")]
    bibkeys: Vec<String>,
    #[builder(default = r#"String::from("viewapi")"#)]
    jscmd: String,
}

impl OpenlibraryRequest for BooksGeneric {
    fn path(&self) -> String {
        String::from("/api/books")
    }

    fn query(&self) -> Vec<(&'static str, String)> {
        vec![
            ("format", String::from("json")),
            ("bibkeys", self.bibkeys.join(",")),
            ("jscmd", self.jscmd.clone()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use mockito::{mock, Matcher};
    use serde_json::json;

    use crate::OpenlibraryRequest;

    use super::{BooksBuilder, BooksGenericBuilder};

    #[test]
    fn test_books_execute() {
        let books = BooksBuilder::default().id("1234").build().unwrap();

        let json = json!({
            "title": "test",
            "description": "this is a test",
            "key": "/works/1234"
        });

        let _m = mock("GET", Matcher::Regex(format!(r"{}\w*", books.url().path())))
            .with_header("content-type", "application/json")
            .with_body(json.to_string())
            .create();

        let books_result = books.execute();

        assert_eq!(books_result["title"], "test");
        assert_eq!(books_result["description"], "this is a test");
        assert_eq!(books_result["key"], "/works/1234");
    }

    #[test]
    fn test_books_generic_execute() {
        let books_generic = BooksGenericBuilder::default()
            .bibkeys(vec!["ISBN:0385472579".to_string()])
            .build()
            .unwrap();

        let json = json!({
            "ISBN:0385472579": {
                "bib_key": "ISBN:0385472579",
                "preview": "noview",
                "thumbnail_url": "https://covers.openlibrary.org/b/id/240726-S.jpg",
                "preview_url": "https://openlibrary.org/books/OL1397864M/Zen_speaks",
                "info_url": "https://openlibrary.org/books/OL1397864M/Zen_speaks"
            }
        });

        let _m = mock(
            "GET",
            Matcher::Regex(format!(r"{}\w*", books_generic.url().path())),
        )
        .with_header("content-type", "application/json")
        .with_body(json.to_string())
        .create();

        let result = books_generic.execute();
        let inner_result = &result["ISBN:0385472579"];

        assert_eq!(inner_result["bib_key"], "ISBN:0385472579");
        assert_eq!(inner_result["preview"], "noview");
        assert_eq!(
            inner_result["thumbnail_url"],
            "https://covers.openlibrary.org/b/id/240726-S.jpg"
        );
        assert_eq!(
            inner_result["preview_url"],
            "https://openlibrary.org/books/OL1397864M/Zen_speaks"
        );
        assert_eq!(
            inner_result["info_url"],
            "https://openlibrary.org/books/OL1397864M/Zen_speaks"
        );
    }
}
