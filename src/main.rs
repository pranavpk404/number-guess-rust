use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    // Making an empty string which is mutable
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // taking the input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("secret_number:{secret_number}");
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Small"),
        Ordering::Equal => println!("You Win"),
        Ordering::Greater => println!("Small"),
    }
}
