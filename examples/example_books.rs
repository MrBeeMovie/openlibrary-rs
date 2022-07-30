use openlibrary_rs::books::{BookType, BooksBuilder};

fn main() {
    let books = BooksBuilder::default()
        .book_type(BookType::Works)
        .id("OL45883W")
        .build()
        .unwrap();

    println!("{:#?}", books.execute());
}
