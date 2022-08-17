use std::fmt::Display;

use derive_builder::Builder;

use crate::OpenlibraryRequest;

#[derive(Default, Clone, Debug)]
pub enum SearchType {
    #[default]
    Books,
    Authors,
    Subjects,
    Lists,
}

impl Display for SearchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Books => "",
                Self::Authors => "/authors",
                Self::Subjects => "/subjects",
                Self::Lists => "/lists",
            }
        )
    }
}

/// The struct representation of a request to the [Search API](https://openlibrary.org/dev/docs/api/search)
///
/// The fields of this struct are private. If you want to view available fields that can be set please look at the [`SearchBuilder`] struct.
/// For more information on query strings and examples please view [Openlibrary's documentation](https://openlibrary.org/search/howto).
#[derive(Builder, Default, Debug)]
#[builder(setter(into), default)]
pub struct Search {
    #[builder(setter(strip_option))]
    query: Option<String>,
    search_type: SearchType,
    #[builder(default = "1")]
    page: u32,
    #[builder(default = "10")]
    limit: u32,
    #[builder(default = "vec![]")]
    fields: Vec<String>,
}

impl OpenlibraryRequest for Search {
    fn path(&self) -> String {
        format!("/search{}.json", self.search_type)
    }

    fn query(&self) -> Vec<(&'static str, String)> {
        let mut params = Vec::new();
        params.push(("page", self.page.to_string()));
        params.push(("limit", self.limit.to_string()));
        params.push(("q", self.query.as_deref().unwrap_or_default().to_string()));
        params.push(("fields", self.fields.join(",")));

        params
    }
}

#[cfg(test)]
mod tests {
    use mockito::mock;
    use serde_json::json;

    use crate::OpenlibraryRequest;

    use super::SearchBuilder;

    #[test]
    fn test_search_execute_valid_response() {
        let search = SearchBuilder::default()
            .query("test")
            .fields(
                ["key", "title"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
            .build()
            .unwrap();

        let json = json!({
                "numFound": 1,
                "start": 0,
                "numFoundExact": true,
                "docs": [
                    {
                        "key": "/works/43242",
                        "title": "test",
                    }
                ]
        });

        let _m = mock(
            "GET",
            format!("{}?{}", search.url().path(), search.url().query().unwrap()).as_str(),
        )
        .with_header("content-type", "application/json")
        .with_body(json.to_string())
        .create();

        let search_result = search.execute();

        assert_eq!(search_result["numFound"], 1);
        assert_eq!(search_result["start"], 0);
        assert_eq!(search_result["numFoundExact"], true);
        assert_eq!(search_result["docs"].as_array().unwrap().len(), 1);

        let doc = &search_result["docs"][0];

        assert_eq!(doc.get("key").unwrap().as_str().unwrap(), "/works/43242");
        assert_eq!(doc.get("title").unwrap().as_str().unwrap(), "test");
    }
}
