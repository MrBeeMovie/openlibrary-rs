use regex::Regex;
use scraper::{Html, Selector};
use std::fmt::Display;

const GOODREADS_URL: &str = "https://www.goodreads.com";

#[derive(Default, Display)]
#[allow(dead_code)]
pub enum SearchField {
    #[default]
    All,
    Title,
    Author,
    Genre,
}

#[derive(Default, Debug)]
pub struct Book {
    url: String,
    title: String,
    description: String,
}

impl Book {
    pub fn new(url: String) -> Book {
        Book {
            url,
            ..Default::default()
        }
    }
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
            GOODREADS_URL, self.search_term, self.search_field
        )
        .to_lowercase()
    }

    fn get_books(body: String) -> Vec<Book> {
        let html = Html::parse_document(body.as_str());
        let book_list_selector = Selector::parse("tr").unwrap();

        let book_list = html.select(&book_list_selector);
        let mut links = Vec::new();

        let re = Regex::new(r"/book/show/\d*").expect("Failed to create regex.");

        for element in book_list {
            let selector = Selector::parse("a").unwrap();
            let element = element.select(&selector).next().unwrap();

            let href = element.value().attr("href");

            match href {
                Some(href) => {
                    match re.find(href) {
                        Some(href) => {
                            links.push(Book::new(format!("{}{}", GOODREADS_URL, href.as_str())))
                        }
                        None => (),
                    };
                }
                None => (),
            }
        }

        links
    }

    pub fn execute(&self) -> Vec<Book> {
        let response = ureq::get(self.get_url().as_str())
            .call()
            .expect("Failed to perform get request.");

        let body = response
            .into_string()
            .expect("Failed to get body as string.");

        Query::get_books(body)
    }
}
