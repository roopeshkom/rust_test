extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Sets a random number between 1-100 as answer
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Allows player to guess until number is found
    loop {
        println!("Please input your guess below:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
