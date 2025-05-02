use std::io;

fn main() {
    let mut name: String = String::new();
    let mut age_input: String = String::new();
    let mut city: String = String::new();

    println!("What's your name?");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");

    println!("What's your age?");
    io::stdin().read_line(&mut age_input).expect("Failed to read age");

    let age: i32 = age_input.trim().parse().expect("Not a valid number");

    println!("What's your city?");
    io::stdin()
        .read_line(&mut city)
        .expect("Failed to read city");

    println!(
        "Hi {}, you're {} years old and you're from {} city.",
        name.trim(), age, city.trim()
    )
}
