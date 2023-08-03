use std::fs::{self, File};
use std::io::{self, ErrorKind, Read, Write};

fn main() {
    println!("h");
    let greeting_file_result = File::open("hello.txt");
    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(err) => panic!("Could not create the file! {:?}", err),
            },
            other_error => {
                panic!("Could not open the file! {:?}", other_error)
            }
        },
    };

    println!("File : {:?}", greeting_file);
    File::create("hello.txt").unwrap().write_all(b"Hello, I am learning Rust\n This is an exciting programming language! \n And I love it ! \n").expect("Couldn't write to file");
    let mut file_contents: String = String::new();
    greeting_file
        .read_to_string(&mut file_contents)
        .expect("Could not load all the contents to string");

    println!("Contents: {} ", file_contents);

    // let greeting_file = File::options()
    //     .write(true)
    //     .open("hello.txt")
    //     .unwrap_or_else(|error| {
    //         if error.kind() == ErrorKind::NotFound {
    //             File::create("hello.txt")
    //                 .unwrap_or_else(|error| panic!("Problem creating the file {:?}", error))
    //         } else {
    //             panic!("Problem opening the file, {:?}", error)
    //         }
    //     });
    let mut whole_text = String::new();
    match read_username_from_file() {
        Ok(username) => {
            println!("Found username, {}", username);
            whole_text = username
        }
        Err(e) => {
            println!("handling the error as we are supposed to! {}", e);
        }
    }

    match first_line_last_char(&whole_text.as_str()) {
        Some(first_char) => println!("last char is, {first_char}"),
        None => println!("None to display"),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let username_file_result = File::open("hello.txt");
    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(error) => return Err(error),
    // };

    // let mut username = String::new();
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username);
    // Ok(username)

    fs::read_to_string("hello.txt")
}

fn first_line_last_char(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
