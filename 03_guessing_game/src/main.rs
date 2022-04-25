/*
We will learn :

    taking inputs

    Datatypes

    compare two values using Ordering

    Looping

*/

use rand::Rng;
use std::cmp::Ordering;
use std::io; // -> for taking inputs

fn main() {
    println!("Guess the number between 1 to 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mut ->  mutable variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!"); // shadowing the previous variable
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                //  The underscore, _ , is a catchall value;
                println!("Please input valid number");
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
