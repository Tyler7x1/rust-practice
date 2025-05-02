struct Book {
    title: String,
    author: String,
    pages: u32,
    read: bool,
}

impl Book {
    fn calculate_cost(&self) -> f64 {
        self.pages as f64 * 0.90
    }
    fn compare_pages(&self, other: &Book) -> bool {
        self.pages > other.pages
    }
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
    println!("Publishing Cost: $ {:.2}", book1.calculate_cost());
    print_summary(&book2);
    println!("Publishing Cost: $ {:.2}", book2.calculate_cost());
    let result: bool = book1.compare_pages(&book2);
    if result { print!("book1 has more pages than book2."); } else { print!("book2 has more pages than book1."); }
    println!()
}

fn print_summary(book: &Book) {
    let status: &str = if book.read { "Read" } else { "Unread" };
    println!("Title: {} | Author: {} | Pages: {} | Status: {}", book.title, book.author, book.pages, status);
}