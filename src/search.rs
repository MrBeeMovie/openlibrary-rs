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
            search.page,
            search.limit,
            search.fields.join(",")
        )
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SearchDoc {
    pub key: String,
    pub r#type: String,
    pub has_fulltext: bool,
    pub title: String,
    pub title_suggest: String,
    pub title_sort: String,
    pub subtitle: String,
    pub edition_count: u32,
    pub edition_key: Vec<String>,
    pub cover_edition_key: String,
    pub publish_date: Vec<String>,
    pub publish_year: Vec<u32>,
    pub first_publish_year: u32,
    pub language: Vec<String>,
    pub number_of_pages_median: u32,
    pub lccn: Vec<String>,
    pub ia: Vec<String>,
    pub ia_box_id: Vec<String>,
    pub ia_loaded_id: Vec<String>,
    pub ia_collection_s: String,
    pub oclc: Vec<String>,
    pub isbn: Vec<String>,
    pub lcc: Vec<String>,
    pub ddc: Vec<String>,
    pub contributor: Vec<String>,
    pub publish_place: Vec<String>,
    pub publisher: Vec<String>,
    pub first_sentence: Vec<String>,
    pub author_key: Vec<String>,
    pub author_name: Vec<String>,
    pub author_alternative_name: Vec<String>,
    pub subject: Vec<String>,
    pub place: Vec<String>,
    pub time: Vec<String>,
    pub seed: Vec<String>,

    // author fields
    pub name: String,
    pub alternate_names: Vec<String>,
    pub birth_date: String,
    pub death_date: String,
    pub date: String,
    pub work_count: u32,
    pub top_work: String,
    pub top_subjects: Vec<String>,

    // subject fields
    pub subject_type: String,

    // dynamic fields
    pub id_goodreads: Vec<String>,
    pub id_amazon: Vec<String>,
    pub id_google: Vec<String>,
    pub id_wikidata: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct SearchResult {
    pub num_found: u32,
    pub start: u32,
    pub num_found_exact: bool,
    pub docs: Vec<SearchDoc>,
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
