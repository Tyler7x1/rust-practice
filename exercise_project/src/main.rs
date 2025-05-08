use exercise_project::{math, string_utils};

fn main() {
    let sum = math::add(5, 4);
    let cap_s = string_utils::capitalize("bruh");

    println!("{}", sum);
    println!("{}", cap_s);
}