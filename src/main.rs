use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("------------------- Guess the number -------------------");
    let secret_number = rand::thread_rng().gen_range(1..10);

    loop {
        println!("Please input your guess.");
        // Making an empty string which is mutable
        let mut guess = String::new();
        // taking the input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("secret_number:{secret_number}");
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Small"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
            Ordering::Greater => println!("Great"),
        }
    }
}
