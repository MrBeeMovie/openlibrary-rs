use openlibrary_rs::search::SearchBuilder;

fn main() {
    let results = SearchBuilder::default()
        .query("the lord of the rings")
        .build()
        .unwrap();

    println!("{:#?}", results.execute().docs[0]);
}
