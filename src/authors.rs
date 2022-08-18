use derive_builder::Builder;

use crate::OpenlibraryRequest;

#[derive(Default, Clone, Debug)]
pub enum AuthorsType {
    #[default]
    Data,
    Works,
}

#[derive(Builder, Default, Debug)]
#[builder(setter(into), default)]
pub struct Authors {
    id: String,
    authors_type: AuthorsType,
    #[builder(default = "10")]
    limit: u32,
    offset: u32,
}

impl OpenlibraryRequest for Authors {
    fn path(&self) -> String {
        match self.authors_type {
            AuthorsType::Data => format!("/authors/{}.json", self.id),
            AuthorsType::Works => format!("/authors/{}/works.json", self.id),
        }
    }

    fn query(&self) -> Vec<(&'static str, String)> {
        vec![
            ("limit", self.limit.to_string()),
            ("offset", self.offset.to_string()),
        ]
    }
}
