#[derive(Debug)]

enum Organism {
    Plant(String),
    Animal(String),
}

fn main() {
    let mango = Organism::Plant(String::from("Mango"));
    let monkey = Organism::Animal(String::from("Monkey"));
    println!("{:?}", mango);
    println!("{:?}", monkey);
}