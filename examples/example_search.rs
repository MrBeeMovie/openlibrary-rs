use openlibrary_rs::search::SearchBuilder;

fn main() {
    let search = SearchBuilder::default()
        .query("the lord of the rings")
        .build()
        .unwrap();

    println!("{:#?}", search.execute().docs[0]);
}
