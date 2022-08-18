use openlibrary_rs::{authors::AuthorsBuilder, OpenlibraryRequest};

fn main() {
    let authors = AuthorsBuilder::default().id("OL23919A").build().unwrap();

    println!("{:#?}", authors.execute());
}
