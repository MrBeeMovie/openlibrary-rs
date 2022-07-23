pub mod book;

use serde_derive::Deserialize;

const OPENLIBRARY_URL: &str = "https://openlibrary.org";

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SearchDoc {
    key: String,
    title: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SearchResult {
    num_found: u32,
    start: u32,
    num_found_exact: bool,
    docs: Vec<SearchDoc>,
    q: String,
}

#[derive(Default, Debug)]
pub struct Search {
    search_term: String,
    search_fields: Vec<String>,
}

impl Search {
    pub fn new(search_term: &str, search_fields: Vec<&str>) -> Search {
        let search_term = String::from(search_term);
        let search_fields = search_fields.into_iter().map(String::from).collect();

        Search {
            search_term,
            search_fields,
        }
    }

    pub fn execute(&self) -> SearchResult {
        let search_fields = self.search_fields.join(",");

        let response = reqwest::blocking::get(format!(
            "{}/search.json?q={}&fields={}",
            OPENLIBRARY_URL, self.search_term, search_fields
        ))
        .unwrap();

        response.json().unwrap()
    }
}
