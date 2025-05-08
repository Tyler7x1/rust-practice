#[derive(Debug)]

enum OrganismKind {
    Plant,
    Animal,
}

struct Organism {
    category: OrganismKind,
    lifespan: u32,
}

fn main() {
    let mango = Organism{
        category: OrganismKind::Plant,
        lifespan: 20,
    };

    let monkey = Organism{
        category: OrganismKind::Animal,
        lifespan: 25,
    };

    println!("{:?}", mango);
    println!("{:?}", monkey);
}