struct Book {
    title: String,
    author: String,
    pages: u32,
    read: bool,
}

impl Book {
    fn new(title: &str, author: &str, pages: u32, read: bool) -> Book {
        Book {
            title: String::from(title),
            author: String::from(author),
            pages,
            read,
        }
    }
    fn calculate_cost(&self) -> f64 {
        self.pages as f64 * 0.90
    }
    fn compare_pages(&self, other: &Book) -> bool {
        self.pages > other.pages
    }
}

fn main() {

    let book1 = Book::new("The Rust Book", "Ferris", 320, true);
    let book2 = Book::new("XYZ", "ABC", 250, false);
    print_summary(&book1);
    println!("Publishing Cost: $ {:.2}", book1.calculate_cost());
    print_summary(&book2);
    println!("Publishing Cost: $ {:.2}", book2.calculate_cost());

    if book1.compare_pages(&book2) {
        println!("book1 has more pages than book2.");
    } else {
        println!("book2 has more pages than book1.");
    }
}

fn print_summary(book: &Book) {
    let status: &str = if book.read { "Read" } else { "Unread" };
    println!("Title: {} | Author: {} | Pages: {} | Status: {}", book.title, book.author, book.pages, status);
}