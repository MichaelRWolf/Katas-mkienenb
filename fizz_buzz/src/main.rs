use std::io;

fn fizz_buzz(numbers: Vec<i32>) -> Vec<String> {
    numbers.iter().map(|i| {
        if *i % 15 == 0 {
            "Fizz Buzz".to_string()
        } else if *i % 5 == 0 {
            "Buzz".to_string()
        } else if *i % 3 == 0 {
            "Fizz".to_string()
        } else {
            i.to_string()
        }
    }).collect()
 }

 fn main() {
    // Get user input
    println!("Enter a number:");
 
    let mut input = String::new();
 
    let _err = io::stdin().read_line(&mut input);
 
    let input: i32 = input.trim().parse().unwrap_or(0);
 
    // Create a range from 1 to the user's input
    let numbers = (1..=input).collect::<Vec<i32>>();
 
    // Run fizz_buzz function
    let result = fizz_buzz(numbers);
 
    // Print result
    for n in result {
        println!("{}", n);
    }
 }