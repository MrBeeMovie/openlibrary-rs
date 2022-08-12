use openlibrary_rs::search::SearchBuilder;
use openlibrary_rs::OpenlibraryRequest;

fn main() {
    let search = SearchBuilder::default()
        .query("the lord of the rings")
        .build()
        .unwrap();

    println!("{:#?}", search.execute());
}
