use derive_builder::Builder;
use serde_json::Value;
use std::{collections::HashMap, fmt::Display};

use crate::OpenlibraryRequest;

pub struct BooksResult {
    pub fields: Vec<HashMap<String, Value>>,
}

#[derive(Default, Clone, Debug)]
pub enum BookType {
    #[default]
    Works,
    Editions,
    ISBN,
}

impl Display for BookType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Works => "/works",
                Self::Editions => "/books",
                Self::ISBN => "/isbn",
            }
        )
    }
}

#[derive(Builder, Default, Debug)]
#[builder(setter(into), default)]
pub struct Books {
    pub book_type: BookType,
    pub id: String,
}

impl Books {
    pub fn execute(&self) -> HashMap<String, Value> {
        let request = OpenlibraryRequest::books_request(self);
        let response = request.execute().unwrap();

        response.json().unwrap()
    }
}
