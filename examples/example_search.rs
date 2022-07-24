use openlibrary_rs::search::SearchBuilder;

fn main() {
    let results = SearchBuilder::default()
        .query("Harry Potter")
        .build()
        .unwrap();

    println!("{:?}", results.execute());
}
