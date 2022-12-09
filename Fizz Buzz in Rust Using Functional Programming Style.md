# Fizz Buzz in Rust Using Functional Programming Techniques

Rust is a powerful modern language that allows for writing efficient and reliable code. Functional programming techniques allow for writing code in a concise and declarative way that is easier to reason about. This can be especially helpful when writing programs for solving puzzles like Fizz Buzz. 

In this tutorial, we'll cover the basics of creating a Fizz Buzz program in Rust using functional programming techniques. 

## What is Fizz Buzz? 

Fizz Buzz is a popular programming game in which a player counts up from a given number and at specific intervals (commonly 3 and 5) substitutes the number with the word "Fizz" and "Buzz" instead. For instance, if the starting number is 1, the sequence would be:

1, 2, Fizz, 4, Buzz, Fizz, 7, 8, Fizz, Buzz, 11, Fizz, 13, 14, Fizz Buzz, etc. 

## Writing the Program

To get started, let's create a new directory and use [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to create a new Rust project. 

Once we have our project set up, we'll want to create a new `main.rs` file in the project directory to write our program. 

We'll need to import the `std::io` library, which will allow us to get user input from the command line. We'll also need the `std::iter` library, which will give us access to the `map()` method for mapping a function to a collection of values. 

```rust
use std::io;
use std::iter;
```

Now we'll define our `fizz_buzz` function, which will take an array of numbers and return an array of strings according to the Fizz Buzz rules. 

```rust
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
```

The `map()` method takes a closure (a function) as a parameter and applies that function to each element in a collection. In this case, we are passing in a function that takes an `i32` (an integer) and returns a `String`. The function checks to see if the number is divisible by 3, 5, or 15, and returns the appropriate string.

Now that we have our `fizz_buzz` function, we'll write a `main` function to get user input and run the `fizz_buzz` function. 

```rust
fn main() {
   // Get user input
   println!("Enter a number:");

   let mut input = String::new();

   io::stdin().read_line(&mut input);

   let input: i32 = match input.trim().parse() {
       Ok(num) => num,
       Err(_) => 0,
   };

   // Create a range from 1 to the user's input
   let numbers = (1..=input).collect::<Vec<i32>>();

   // Run fizz_buzz function
   let result = fizz_buzz(numbers);

   // Print result
   for n in result {
       println!("{}", n);
   }
}
```

The `main` function starts by getting user input from the command line and then creates a range from 1 to the user's input. We then pass this range of numbers to our `fizz_buzz` function, which returns an array of strings. Finally, we print out the result. 

And that's it! Our Fizz Buzz program is complete. 

Run the program with `cargo run` and you should see the Fizz Buzz sequence printed out in the terminal. 

Congratulations, you have now created a Fizz Buzz program in Rust using functional programming techniques!