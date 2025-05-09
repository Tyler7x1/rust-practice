use lib::traits::{Book, Describable, notify};

mod lib {
    pub mod traits;
}

fn main() {
    let book = Book {
        title: String::from("1984"),
        author: String::from("George Orwell"),
    };
    println!("{}", book.describe());
    notify(&book);
}