use std::{cmp::Ordering, vec};

fn main() {
    let mut v = vec![1, 2, 3, 4];
    v.push(89);
    println!("first element is : {}", &v[0]);

    let fourth = v.get(3);
    // println!("fourth element is {:?}", fourth);
    match fourth {
        Some(fourth) => println!("element is {fourth}"),
        None => println!("Element could not be found."),
    }
    // v.pop();
    // v.remove(3);
    v.retain(|&x| x == 89);
    println!("element: {}", &v[0]);
    println!("len of vector is: {}", v.len());

    if let Ordering::Equal = &v[0].cmp(&89) {
        println!("Exact match!")
    }
    let s = "Здравствуйте";
    println!("length of Здравствуйте is {}", s.len());

    println!("printing chars() of Здравствуйте: ");
    for c in s.chars() {
        println!("{c}");
    }

    println!("printing bytes() of Здравствуйте: ");
    for b in s.bytes() {
        println!("{b}");
    }

    println!("printing as_bytes() of Здравствуйте: ");
    for b in s.as_bytes() {
        println!("{b}");
    }
}
