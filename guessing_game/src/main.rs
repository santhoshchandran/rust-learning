extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!"); //All macros ends with !

    let secret_number = rand::thread_rng().gen_range(1, 101); //thread_rng returns ThreadRng which is an instance of Rng trait

    loop {

        println!("Please input your guess"); 

        let mut guess = String::new(); //Creates new instance of type String

        io::stdin().read_line(&mut guess) //From io crate, call read_line on stdin instance and assign it to mutable guess reference
            .expect("Failed to read line");

        // Trims guess and parse it to u32 datatype, which returns enum Result(Ok, Err)
        // match is almost similar to case in JAVA
        // Ok takes num attribute which we assign it back to guess
        // Err - underscore, handles all types of exception 
        let guess: u32 = match guess.trim().parse() { 
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Compares guess to secret_number
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
