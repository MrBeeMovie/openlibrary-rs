use openlibrary_rs::books::{BookType, BooksBuilder};

fn main() {
    let books = BooksBuilder::default()
        .book_type(BookType::ISBN)
        .id("9780140328721")
        .build()
        .unwrap();

    println!("{:#?}", books.execute());
}
