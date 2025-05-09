#[derive(Debug)]

pub struct Book {
    pub title: String,
    pub author: String,
}

pub fn add_book(title: &str, author: &str) -> Book {
    Book {
        title: title.to_string(),
        author: author.to_string(),
    }
}