
use std::io;
use std::cmp::Ordering;
use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use colored::*;

fn main() {
    println!("Welcome to Guess the number!");

    let secret_number = rand::thread_rng().sample(Uniform::new_inclusive(1, 100));

    println!("The secret_number is {}", secret_number);


    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };

        println!("You guessed: {}", guess);


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}