mod library;

use library::{books, members};

fn main() {
    let book = books::add_book("The Rust Book", "Ferris Crab");
    let member = members::register("Ravi", 101);

    println!("Book added: {:?}", book);
    println!("Member registered: {:?}", member);
}

