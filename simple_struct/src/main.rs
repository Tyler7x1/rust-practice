struct Book {
    title: String,
    author: String,
    pages: u32,
    read: bool,
}

fn main() {
    let book1: Book = Book {
        title: String::from("The Rust Book"),
        author: String::from("Ferris"),
        pages: 320,
        read: true,
    };
    
    let book2: Book = Book {
        title: String::from("XYZ"),
        author: String::from("ABC"),
        pages: 250,
        read: false,
    };
    print_summary(&book1);
    print_summary(&book2);
}

fn print_summary(book: &Book) {
    let status: &str = if book.read { "Read" } else { "Unread" };
    println!("Title: {} | Author: {} | Pages: {} | Status: {}", book.title, book.author, book.pages, status);
}