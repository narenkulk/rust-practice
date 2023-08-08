use std::fmt::Display;

struct Pair<T>{
    x: T,
    y: T,
}

impl<T> Pair<T>{
    fn new(x: T, y: T) -> Self{
        Self{x,y}
    }
}

impl<T: Display+PartialOrd> Pair<T>{
    fn cmd_display(&self) {
        if self.x > self.y{
            println!("largest member: {}", self.x);
        } else{
            println!("largest member: {}", self.y);
        }
    }
}

fn main() {
    println!("Hello, world!");
    let p = Pair::new("1", "20");
     Pair::cmd_display(&p); 
    // println!("pair with cmd_display: {}", pair_with_cmd_display);
}
