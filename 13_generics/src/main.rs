use generics::{self, Summary}; // this is the name of the module defined in Cargo.toml file

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item
        }
    }
    &largest
}

fn main() {
    let v = vec![4, 7, 18, 3, 98];
    println!("largest item is: {}", largest(&v));

    let c_list = ['a', 'b', 'c', 'y'];

    println!("largest item is: {}", largest(&c_list));

    let tweet = generics::aggregator::tweet::Tweet {
        username: String::from("jk343"),
        content: String::from("Hey! that was my first tweet!"),
        reply: true,
        retweet: false,
    };
    println!("Summary of the tweet: {}", tweet.summarize());
}
