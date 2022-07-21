use regex::Regex;
use scraper::{Html, Selector};
use std::fmt::Display;

use crate::search::book::Book;
use crate::search::GOODREADS_URL;

#[derive(Default, Display)]
#[allow(dead_code)]
pub enum SearchField {
    #[default]
    All,
    Title,
    Author,
    Genre,
}

#[derive(Default)]
pub struct Query {
    pub search_term: String,
    pub search_field: SearchField,
}

impl Query {
    fn get_url(&self) -> String {
        format!(
            "{}/search?q={}&search[field]={}",
            GOODREADS_URL,
            self.search_term,
            self.search_field.to_string().to_lowercase()
        )
    }

    fn get_books(body: String) -> Vec<Book> {
        let html = Html::parse_document(body.as_str());

        // Selector and Regex initialization should never fail
        let tr_selector = Selector::parse("tr").unwrap();
        let a_selector = Selector::parse("a").unwrap();
        let re = Regex::new(r"\d\d*").unwrap();

        html.select(&tr_selector)
            .map(|element_ref| {
                element_ref
                    .select(&a_selector)
                    .next()
                    .expect("No books in search results.")
            })
            .map(|element_ref| {
                element_ref
                    .value()
                    .attr("href")
                    .expect("Missing url for book in search results.")
            })
            .map(|href| {
                re.find(href)
                    .expect("Missing id for book in search results.")
                    .as_str()
                    .parse::<u32>()
                    // this should never happen Regex only matches digits
                    .unwrap()
            })
            .map(|id| Book::new(id))
            .collect();
    }

    pub fn execute(&self) -> Vec<Book> {
        let response = ureq::get(self.get_url().as_str())
            .call()
            .expect("Failed to perform GET request.");

        let body = response
            .into_string()
            .expect("Failed to get body as string.");

        Query::get_books(body)
    }
}

#[cfg(test)]
mod tests {
    use mockito::mock;
    use regex::Regex;

    use super::{Query, SearchField, GOODREADS_URL};

    #[test]
    fn test_query_execute() {
        let query = Query {
            search_term: String::from("test"),
            search_field: SearchField::All,
        };

        let body = include_str!("./resources/goodreads_search_test.html");

        let _m = mock(
            "GET",
            format!(
                "{}/search?q={}&search[field]={}",
                GOODREADS_URL, query.search_field, query.search_term
            )
            .as_str(),
        )
        .with_header("content-type", "application/json")
        .with_body(body)
        .create();

        let re = Regex::new(r"\d\d*").unwrap();

        for book in query.execute() {
            assert!(re.is_match(format!("{}", book.id).as_str()));
            assert_eq!(
                re.find(format!("{}", book.id).as_str()).unwrap().as_str(),
                format!("{}", book.id).as_str()
            );
        }
    }
}
