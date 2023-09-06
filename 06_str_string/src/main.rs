fn main() {
    println!("Hello, world!");
    let s = " hello";
    let s = s.trim();
    println!("{s}");

    let foo = "car";
    let bar = foo.to_string();
    let car = &bar;
    println!("{car}");
    let a = true;
    let b = false;
    let c = true;

    if a && (b || c) {
        println!("hello")
    }
}
