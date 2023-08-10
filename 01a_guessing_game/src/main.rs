use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 10 {
            println!("value must be between 1 and 10, got {value}");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..=10);
    let mut count: u32 = 0;

    loop {
        println!("Enter the number to guess:");
        let mut guess_userinput = String::new();
        io::stdin()
            .read_line(&mut guess_userinput)
            .expect("Couldn't read the number!");

        let mut guess = Guess::new(guess_userinput.trim().parse().unwrap());

        count += 1;

        match secret_number.cmp(&mut guess.value) {
            Ordering::Equal => {
                println!("You got it right in {count} count!!");
                break;
            }
            Ordering::Greater => println!("Secret number is greater than yours"),
            Ordering::Less => println!("Secret number is lesser than yours"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_guess_valid_value() {
        let guess = Guess::new(5);
        assert_eq!(guess.value(), 5);
    }

    // #[test]
    // #[should_panic(expected = "value must be between 1 and 10")]
    // fn test_new_guess_invalid_value() {
    //     Guess::new(11);
    // }
}

