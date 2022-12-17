# CodexPedia reply to "Fizz Buzz in Rust Using OO Programming Techniques"


Fizz Buzz is a classic coding challenge often used to teach programming basics. It consists of writing a program that prints the numbers from 1 to 100, but for multiples of three print “Fizz” instead of the number, and for the multiples of five print “Buzz”. Here is an example of a Rust implementation of Fizz Buzz using object-oriented programming techniques:

```rust
struct FizzBuzz {
    cur: i32,
}

impl FizzBuzz {
    fn new() -> FizzBuzz {
        FizzBuzz { cur: 1 }
    }

    fn next(&mut self) -> String {
        let mut result = String::new();

        if self.cur % 3 == 0 {
            result.push_str("Fizz");
        }
        if self.cur % 5 == 0 {
            result.push_str("Buzz");
        }
        if result.is_empty() {
            result = self.cur.to_string();
        }
        self.cur += 1;
        result
    }
}

fn main() {
    let mut fb = FizzBuzz::new();
    for _i in 0..100 {
        println!("{}", fb.next());
    }
}
```
