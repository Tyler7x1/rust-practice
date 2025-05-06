use std::io;
use std::fmt;

struct Measurement(f64, String);

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

fn main() {
    let mut num_inp1: String = String::new();
    let mut num_inp2: String = String::new();

    let mut str_inp1: String = String::new();
    let mut str_inp2: String = String::new();


    println!("Please enter a number");
    io::stdin().read_line(&mut num_inp1).expect("Failed to read value");
    let num1: f64 = num_inp1.trim().parse().expect("Expected a number!");

    println!("Please enter another number");
    io::stdin().read_line(&mut num_inp2).expect("Failed to read value");
    let num2: f64 = num_inp2.trim().parse().expect("Expected a number!");

    println!("Please enter a string");
    io::stdin().read_line(&mut str_inp1).expect("Failed to read value");
    let str1: String = str_inp1.trim().to_string();

    println!("Please enter another string");
    io::stdin().read_line(&mut str_inp2).expect("Failed to read value");
    let str2: String = str_inp2.trim().to_string();

    let width1 = Measurement(num1, str1);
    println!("Measurement: {} {}", width1.0, width1.1);

    let width2 = Measurement(num2, str2);
    println!("Measurement: {} {}", width2.0, width2.1);

    let result: String = compare_measurement(&width1, &width2).to_string();
    println!("{}", result);
}

fn compare_measurement(m1: &Measurement, m2: &Measurement) -> String {
    if m1.0 > m2.0 {
        format!("{} is greater than {}", m1, m2)
    } else if m1.0 < m2.0 {
        format!("{} is greater than {}", m2, m1)
    } else {
        format!("{} and {} are equal", m1, m2)
    }
}