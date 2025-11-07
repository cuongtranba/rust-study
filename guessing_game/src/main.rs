use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(1..=100);
    let mut guess = String::new();
    loop {
        println!("please input your guess.");
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!, secret number is {}", secret_number),
            Ordering::Greater => println!("Too big!, secret number is {}", secret_number),
            Ordering::Equal => {
                println!("You win!, secret number is {}", secret_number);
                break;
            }
        }
    }
}
