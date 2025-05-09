fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Exercise 1
    for num in &numbers {
        println!("{num}");
    }

    // Exercise 2
    let mapped_numbers: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("Applied map method: {:?}", mapped_numbers);

    // Exercise 3
    let filtered_numbers: Vec<&i32> = numbers.iter().filter(|x| **x % 2 == 0).collect();
    println!("Applied filter method: {:?}", filtered_numbers);

    // Exercise 4
    let sum: i32 = numbers.iter().sum::<i32>(); // OR: numbers.iter().copied().sum()
    println!("Sum: {}", sum);

    // Exercise 5
    let factorial: i32 = numbers.iter().fold(1, |acc, &curr| acc * curr);
    println!("Factorial: {}", factorial);

    // Exercise 6
    let map_and_filter: Vec<i32> = numbers
        .iter()
        .map(|&x| x * 2)
        .filter(|x| x % 2 == 0)
        .collect();
    println!("Map + Filter: {:?}", map_and_filter);

    // Exercise 7
    let mut peekable = numbers.iter().peekable();
    println!("Peek: {:?}", peekable.peek());

    // Exercise 8
    let iter_2: Vec<i32> = vec![6, 7, 8];
    let zip: Vec<(&i32, &i32)> = numbers.iter().zip(iter_2.iter()).collect();
    println!("Zip: {:?}", zip);

    // Exercise 9
    let infinite: Vec<i32> = (1..).take(3).collect();
    println!("Only 3 numbers from infinite: {:?}", infinite);
}
