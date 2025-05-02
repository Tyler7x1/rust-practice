use std::io;

fn main() {
    let mut user_inp: String = String::new();

    println!("**Convert temperature to Fahrenheit/Celsius.**");
    println!(
        "Type C to convert temperature into Fahrenheit and F to convert temperature into Celsius."
    );

    io::stdin()
        .read_line(&mut user_inp)
        .expect("Failed to read input.");

    let user_inp = user_inp.trim().to_uppercase();

    if user_inp == "C" {
        let mut temp: String = String::new();
        println!("Enter temperature in Celsius: ");
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read temperature.");
        let c: f64 = temp.trim().parse().expect("Expected a number!");
        let f: f64 = (c * 9.0 / 5.0) + 32.0;
        println!("{:.2} C converts to {:.2} F", c, f);
    } else if user_inp == "F" {
        let mut temp: String = String::new();
        println!("Enter temperature in Fahrenheit: ");
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read temperature.");
        let f: f64 = temp.trim().parse().expect("Expected a number!");
        let c: f64 = (f - 32.0) * (5.0 / 9.0);
        println!("{:.2} F converts to {:.2} C", f, c);
    } else {
        println!("Invalid Input! Please enter 'C' or 'F'.");
    }
}
