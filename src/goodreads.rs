use scraper::{Html, Selector};
use std::fmt::Display;

#[derive(Default, Display)]
#[allow(dead_code)]
pub enum SearchField {
    #[default]
    All,
    Title,
    Author,
    Genre,
}

#[derive(Default, Display)]
#[allow(dead_code)]
pub enum SearchType {
    #[default]
    Books,
    Groups,
    Quotes,
    Stories,
    People,
    Listopia,
    Trivia,
}

#[derive(Default)]
pub struct Query {
    pub search_term: String,
    pub search_field: SearchField,
    pub search_type: SearchType,
}

impl Query {
    fn get_url(&self) -> String {
        format!(
            "https://www.goodreads.com/search?q={}&search[field]={}&search_type={}",
            self.search_term, self.search_field, self.search_type,
        )
        .to_lowercase()
    }

    fn get_books(&self, body: String) -> Vec<String> {
        let html = Html::parse_document(body.as_str());
        let book_list_selector = Selector::parse("tr").unwrap();

        let book_list = html.select(&book_list_selector);
        let mut links = Vec::new();

        for element in book_list {
            let selector = Selector::parse("a").unwrap();
            let element = element.select(&selector).next().unwrap();

            let href = element.value().attr("href");

            match href {
                Some(href) => links.push(String::from(href)),
                None => (),
            }
        }

        links
    }

    pub fn execute(&self) -> Vec<String> {
        let response = ureq::get(self.get_url().as_str())
            .call()
            .expect("Failed to perform get request.");

        let body = response
            .into_string()
            .expect("Failed to get body as string.");

        match self.search_type {
            SearchType::Books => self.get_books(body),
            _ => vec![],
        }
    }
}
