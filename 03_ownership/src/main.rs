fn main() {
    let _s1 = gives_ownership(); // s1 gets ownership from the func
    let s2 = String::from("world!");
    let s3 = takes_and_gives_ownership(s2); //s2 goes out of scope here
    println!("{s3}")
}

fn gives_ownership() -> String {
    let s4 = String::from("hello");
    return s4;
}

fn takes_and_gives_ownership(some_string: String) -> String {
    println!("{some_string}");
    let s5 = String::from("hello {some_string}");
    s5
}
