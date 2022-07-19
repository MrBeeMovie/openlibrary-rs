#[macro_use]
extern crate enum_display_derive;

mod goodreads;

use crate::goodreads::Query;

fn main() {
    let query = Query {
        search_term: String::from("Harry Potter"),
        ..Default::default()
    };

    println!("{:?}", query.execute());
}
