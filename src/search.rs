use std::default;

use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};

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
            search.page.unwrap_or_default(),
            search.limit.unwrap_or_default(),
            search
                .fields
                .as_deref()
                .unwrap_or_default()
                .join(",")
                .as_str()
        )
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
#[serde(default)]
pub struct SearchDoc {
    key: String,
    title: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
#[serde(default)]
pub struct SearchResult {
    num_found: u32,
    start: u32,
    num_found_exact: bool,
    docs: Vec<SearchDoc>,
    q: String,
}

#[derive(Builder, Default, Debug)]
#[builder(setter(into, strip_option), default)]
pub struct Search {
    query: Option<String>,
    title: Option<String>,
    author: Option<String>,
    #[builder(default = "Some(1)")]
    page: Option<u32>,
    #[builder(default = "Some(10)")]
    limit: Option<u32>,
    fields: Option<Vec<String>>,
}

impl Search {
    pub fn execute(&self) -> SearchResult {
        let response = reqwest::blocking::get(openlibrary_request::search_url(self)).unwrap();

        response.json().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use mockito::mock;
    use serde_json::{json, Value};

    use super::{Search, SearchResult};

    fn get_search_result(search_term: &str, search_fields: Vec<&str>, json: Value) -> SearchResult {
        let search = Search::new(search_term, search_fields.clone());
        let search_fields = search_fields.join(",");

        let _m = mock(
            "GET",
            format!("/search.json?q={}&fields={}", search_term, search_fields).as_str(),
        )
        .with_header("content-type", "application/json")
        .with_body(json.to_string())
        .create();

        search.execute()
    }

    #[test]
    fn test_search_execute_valid_response() {
        let search_term = "test";
        let search_fields = vec!["key", "title"];

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

        let search_result = get_search_result(search_term, search_fields, json);

        assert_eq!(search_result.num_found, 1);
        assert_eq!(search_result.start, 0);
        assert_eq!(search_result.num_found_exact, true);
        assert_eq!(search_result.docs.len(), 1);

        let doc = &search_result.docs[0];

        assert_eq!(doc.key, "/works/43242");
        assert_eq!(doc.title, "test");
    }
}
