use openlibrary_rs::books::{BookType, BooksBuilder, BooksGenericBuilder};
use openlibrary_rs::OpenlibraryRequest;

fn main() {
    let books = BooksBuilder::default()
        .book_type(BookType::Works)
        .id("OL45883W")
        .build()
        .unwrap();

    println!("{:#?}", books.execute());

    let books_generic = BooksGenericBuilder::default()
        .bibkeys(vec![
            "ISBN:0201558025".to_string(),
            "LCCN:93005405".to_string(),
        ])
        .build()
        .unwrap();

    println!("{:#?}", books_generic.execute());
}
