use std::io;

fn main() {
    let mut number = String::new();
    // Divisible by a number:
    println!("Please enter a number: ");
    io::stdin()
        .read_line(&mut number)
        .expect("Don't seem to be a number!");
    let number: u32 = number.trim().parse().expect("Can't parse it to int");

    if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else if number % 5 == 0 {
        println!("Number is divisible by 5");
    } else {
        println!("Number is neither divisible by 2, 3 or 5");
        println!("Nu");
    }

    // Index
    let arr = [10, 20, 30, 40, 50];
    let mut index = String::new();
    println!("Please enter the index: ");
    io::stdin()
        .read_line(&mut index)
        .expect("Can't read the index");

    let index: usize = index.trim().parse().expect("Can't parse it to integer.");
    let element: u32 = arr[index];
    println!("Your element is: {element}");
    println!("dddd");
    print!("");
}
