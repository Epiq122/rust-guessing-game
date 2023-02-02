// Input/Output from the standard library
use std::io;
// Defines the method for our random number
use rand::Rng;
// Comparing
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100); // thread_rng is the particular funcion in the the ran we are going to use, then generate a range from 1-100
    println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess");

        let mut guess = String::new(); // Lets us know we will be introducting a new mutable variable guess

        io::stdin() // gets the input
            .read_line(&mut guess) // reads what the person tryped in
            .expect("Failed to read line"); // cause the program to crash

        // converting the guess to a u32 int (Shadowing)
        // trim eliminates any white space
        // parse coverts string to another type.

        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed :{guess}");

        match guess.cmp(&secret_number) {
            // compare Gues to the Secret number
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You WIN!");
                break;
            }
        }
    }
}
