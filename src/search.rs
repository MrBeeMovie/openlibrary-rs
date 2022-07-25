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

/// The struct representation of a response from the [Search API](https://openlibrary.org/dev/docs/api/search)
///
/// The available doc fields in the response can be found as a part of [the managed-schema](https://github.com/internetarchive/openlibrary/blob/master/conf/solr/conf/managed-schema#L136-L216) defined in the Openlibrary repository.
/// All doc fields are hashed by key into a [`Vec<HashMap<String, Value>>`].
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

/// The struct representation of a request to the [Search API](https://openlibrary.org/dev/docs/api/search)[^note]
///
/// The fields of this struct are private. If you want to view available fields that you can set please look at the [`SearchBuilder`] struct.
/// For more information on query strings and examples please view [Openlibrary's documentation](https://openlibrary.org/search/howto).
///
/// [^note]: you must use the [`SearchBuilder`] struct to build instance of the [`Search`] struct
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
    /// Function to execute the request defined by the struct and get back an instance of [`SearchResult`]
    ///
    /// Example
    /// ```rust
    /// use openlibrary_rs::search::{SearchBuilder, SearchType};
    ///
    /// let results = SearchBuilder::default()
    ///     .query("the lord of the rings")
    ///     .search_type(SearchType::Books)
    ///     .page(1 as u32)
    ///     .limit(1 as u32)
    ///     .fields(
    ///         vec!["key", "title", "edition_key"]
    ///             .into_iter()
    ///             .map(String::from)
    ///             .collect::<Vec<String>>(),
    ///    )
    ///    .build()
    ///    .unwrap();
    ///
    /// println!("{:#?}", results.execute().docs[0]);
    /// ```
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
