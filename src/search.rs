use std::collections::HashMap;

use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

pub mod openlibrary_request {
    use super::Search;

    #[allow(dead_code)]
    const OPENLIBRARY_URL: &str = "https://openlibrary.org";

    pub fn search_url(search: &Search) -> String {
        #[cfg(not(test))]
        let root_url = OPENLIBRARY_URL.to_string();
        #[cfg(test)]
        let root_url = mockito::server_url().to_string();

        format!(
            "{}/search.json?q={}&title={}&author={}&page={}&limit={}&fields={}",
            root_url,
            search.query.as_deref().unwrap_or_default(),
            search.title.as_deref().unwrap_or_default(),
            search.author.as_deref().unwrap_or_default(),
            search.page,
            search.limit,
            search.fields.join(",")
        )
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct SearchResult {
    pub num_found: u32,
    pub start: u32,
    pub num_found_exact: bool,
    pub docs: Vec<HashMap<String, Value>>,
    pub q: String,
}

#[derive(Builder, Default, Debug)]
#[builder(setter(into, strip_option), default)]
pub struct Search {
    query: Option<String>,
    title: Option<String>,
    author: Option<String>,
    #[builder(default = "1")]
    page: u32,
    #[builder(default = "10")]
    limit: u32,
    #[builder(
        default = r#"vec!["title".to_string(), "key".to_string(), "type".to_string(), "edition_key".to_string()]"#
    )]
    fields: Vec<String>,
}

impl Search {
    pub fn execute(&self) -> SearchResult {
        let url = openlibrary_request::search_url(self);
        let response = reqwest::blocking::get(url).unwrap();

        response.json().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use mockito::mock;
    use serde_json::{json, Value};

    use super::{Search, SearchBuilder, SearchResult};

    fn get_search_result(search: Search, json: Value) -> SearchResult {
        let _m = mock(
            "GET",
            format!(
                "/search.json?q={}&title={}&author={}&page={}&limit={}&fields={}",
                search.query.as_deref().unwrap_or_default(),
                search.title.as_deref().unwrap_or_default(),
                search.author.as_deref().unwrap_or_default(),
                search.page,
                search.limit,
                search.fields.join(",")
            )
            .as_str(),
        )
        .with_header("content-type", "application/json")
        .with_body(json.to_string())
        .create();

        search.execute()
    }

    #[test]
    fn test_search_execute_valid_response() {
        let search = SearchBuilder::default()
            .query("test")
            .fields(vec!["key".to_string(), "title".to_string()])
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

        let search_result = get_search_result(search, json);

        assert_eq!(search_result.num_found, 1);
        assert_eq!(search_result.start, 0);
        assert_eq!(search_result.num_found_exact, true);
        assert_eq!(search_result.docs.len(), 1);

        let doc = &search_result.docs[0];

        assert_eq!(doc.key, "/works/43242");
        assert_eq!(doc.title, "test");
    }
}
