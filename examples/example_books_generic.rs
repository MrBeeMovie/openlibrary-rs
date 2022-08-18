use openlibrary_rs::{books::BooksGenericBuilder, OpenlibraryRequest};

fn main() {
    let books_generic = BooksGenericBuilder::default()
        .bibkeys(vec![
            "ISBN:0201558025".to_string(),
            "LCCN:93005405".to_string(),
        ])
        .build()
        .unwrap();

    println!("{:#?}", books_generic.execute());
}
