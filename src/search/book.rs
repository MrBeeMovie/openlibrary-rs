#[derive(Default, Debug, PartialEq)]
pub struct Book {
    pub id: u32,
}

impl Book {
    pub fn new(id: u32) -> Book {
        Book {
            id,
            ..Default::default()
        }
    }
}
