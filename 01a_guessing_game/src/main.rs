use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=10);
    let mut count: u32 = 1;

    loop {
        println!("Enter the number to guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Please enter a number");

        let guess: u32 = match guess.trim().parse() {
            Ok(result) => result,
            Err(_) => {
                println!("You didn't enter a number but ok ");
                continue;
            }
        };

        match &guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win! total guess: {count}");
                break;
            }
            Ordering::Greater => println!("Number is greater"),
            Ordering::Less => println!("Number is lesser"),
        }
        count += 1;
    }
}
