extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 11);
    println!("secret number is {}", secret_number);

    loop {
        let mut guess = String::new();
        println!("Please enter your number: ");
        io::stdin().read_line(&mut guess).expect("could not read");
        let guess: u32 = match guess.trim().parse() {
            Ok(num12) => num12,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("your number is less than secret");
            }
            Ordering::Greater => {
                println!("Your number is greater than secret");
            }
            Ordering::Equal => {
                // println!("You guessed it!!");
                println!("OK");
                break;
            }
        }
    }
}
