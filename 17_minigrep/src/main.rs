use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // cargo run --searchstring filename1.txt

    // let args: Vec<String> = env::args().collect(); // .collect() turns iterator to a list
    // (collection) of strings. we
    // provide Vec<String> to indicate what kind of types we will have in this. Rust wouldn't know
    // coz its done at runtime
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Please pass the query AND file path, {err}");
        process::exit(1);
    });
    // match run(config) {
    //     Err(e) => println!("Error occured while finding the string in file, {e}"),
    //     _ => println!("Found the string!"),
    // };
    if let Err(e) = minigrep::run(config) {
        eprintln!("Error occured while finding the string in file {e}");
        process::exit(1);
    }
    // dbg!(args);
}
