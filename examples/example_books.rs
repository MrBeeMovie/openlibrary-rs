use openlibrary_rs::books::{BookType, BooksBuilder};
use openlibrary_rs::OpenlibraryRequest;

fn main() {
    let books = BooksBuilder::default()
        .book_type(BookType::Works)
        .id("OL45883W")
        .build()
        .unwrap();

    println!("{:#?}", books.execute());
}
