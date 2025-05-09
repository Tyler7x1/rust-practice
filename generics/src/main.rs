use get::Wrapper;

mod get;

fn main() {
    let a = Wrapper { value: 42 };
    let b = Wrapper { value: "hello" };

    println!("a = {}", a.get_value());
    println!("b = {}", b.get_value());
}
