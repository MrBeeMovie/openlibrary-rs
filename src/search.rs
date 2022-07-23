pub mod book;

use serde_derive::{Deserialize, Serialize};

const OPENLIBRARY_URL: &str = "https://openlibrary.org";

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

        #[cfg(not(test))]
        let url = format!(
            "{}/search.json?q={}&fields={}",
            OPENLIBRARY_URL, self.search_term, search_fields
        );
        #[cfg(test)]
        let url = format!(
            "{}/search.json?q={}&fields={}",
            mockito::server_url(),
            self.search_term,
            search_fields
        );

        let response = reqwest::blocking::get(url).unwrap();

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
