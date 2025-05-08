fn main() {
    let three: Option<i32> = Some(3);

    /*match three {
        Some(3) => println!("Correct!"),
        _ => println!("Try again!"),
    };*/

    if let Some(3) = three {
        println!("Correct!");
    } else {
        println!("Try again!");
    }

    let four: Result<i32, String> = Ok(5);

    match four {
        Ok(4) => println!("Correct!"),
        _ => println!("Bruh..."),
        Err(_) => println!("Try again!"),
    }

    println!("{:?}", three); //checking debuggable format display
    println!("{:?}", four); //checking debuggable format display
}
