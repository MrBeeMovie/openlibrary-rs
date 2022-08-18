use derive_builder::Builder;

use crate::OpenlibraryRequest;

#[derive(Default, Clone, Debug)]
pub enum AuthorsType {
    #[default]
    Data,
    Works,
}

/// The struct representation of a request to the [Authors API](https://openlibrary.org/dev/docs/api/authors)
///
/// The fields of this struct are private. If you want to view available fields that can be set please look at the [`AuthorsBuilder`] struct.
#[derive(Builder, Default, Debug)]
#[builder(setter(into), default)]
pub struct Authors {
    id: String,
    authors_type: AuthorsType,
    #[builder(default = "10")]
    limit: u32,
    offset: u32,
}

impl OpenlibraryRequest for Authors {
    fn path(&self) -> String {
        match self.authors_type {
            AuthorsType::Data => format!("/authors/{}.json", self.id),
            AuthorsType::Works => format!("/authors/{}/works.json", self.id),
        }
    }

    fn query(&self) -> Vec<(&'static str, String)> {
        vec![
            ("limit", self.limit.to_string()),
            ("offset", self.offset.to_string()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use mockito::{mock, Matcher};
    use serde_json::json;

    use crate::OpenlibraryRequest;

    use super::AuthorsBuilder;

    #[test]
    fn test_authors_execute() {
        let authors = AuthorsBuilder::default().id("OL23919A").build().unwrap();

        let json = json!({
            "key": "authors/OL23919A",
            "name": "J. K. Rowling",
            "entity_type": "person",
        });

        let _m = mock(
            "GET",
            Matcher::Regex(format!(r"{}\w*", authors.url().path())),
        )
        .with_header("content-type", "application/json")
        .with_body(json.to_string())
        .create();

        let result = authors.execute();

        assert_eq!(result["key"], "authors/OL23919A");
        assert_eq!(result["name"], "J. K. Rowling");
        assert_eq!(result["entity_type"], "person");
    }
}
