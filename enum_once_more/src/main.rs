enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn show_value(&self) {
        match self {
            Coin::Penny => println!("This is a penny"),
            Coin::Nickel => println!("This is a nickel"),
            Coin::Dime => println!("This is a dime"),
            Coin::Quarter => println!("This is a quarter"),
        }
    }
}

fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;

    (&penny).show_value();
    (&nickel).show_value();
    (&dime).show_value();
    (&quarter).show_value();
}