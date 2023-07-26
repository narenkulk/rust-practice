fn main() {
    let s = String::from("world!");
    say_hello(s);
    // println!("{s}"); //Error because s is already copied through func parameter
    let i = 53;
    print_number(i);
}

fn say_hello(some_string: String) {
    println!("hello {some_string}");
}

fn print_number(num: usize) {
    println!("{num}");
}
