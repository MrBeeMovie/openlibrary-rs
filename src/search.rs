use std::{collections::HashMap, fmt::Display};

use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

pub mod openlibrary_request {
    use super::Search;

    #[allow(dead_code)]
    const OPENLIBRARY_URL: &str = "https://openlibrary.org";

    pub fn search_url(search: &Search) -> String {
        #[cfg(not(test))]
        let mut url = OPENLIBRARY_URL.to_string();
        #[cfg(test)]
        let mut url = mockito::server_url().to_string();

        url.push_str("/search");
        url.push_str(search.search_type.to_string().as_str());

        url.push_str(format!(".json?page={}&limit={}", search.page, search.limit,).as_str());

        if let Some(query) = search.query.as_deref() {
            url.push_str(format!("&q={}", query).as_str())
        }

        if !search.fields.is_empty() {
            url.push_str(format!("&fields={}", search.fields.join(",")).as_str())
        }

        url
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
                "/search.json?page={}&limit={}&q={}&fields={}",
                search.page,
                search.limit,
                search.query.as_deref().unwrap_or_default(),
                search.fields.join(","),
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

        let search_result = get_search_result(search, json);

        assert_eq!(search_result.num_found, 1);
        assert_eq!(search_result.start, 0);
        assert_eq!(search_result.num_found_exact, true);
        assert_eq!(search_result.docs.len(), 1);

        let doc = &search_result.docs[0];

        assert_eq!(doc.get("key").unwrap().as_str().unwrap(), "/works/43242");
        assert_eq!(doc.get("title").unwrap().as_str().unwrap(), "test");
    }
}
