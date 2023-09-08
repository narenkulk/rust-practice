use std::ops::Deref;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
struct MyBox<T>(T); // tuple struct

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let b = Box::new(5);
    println!("Box::new(5) is {b}");

    let my_recurrsive_list = List::Cons(
        4,
        Box::new(List::Cons(3, Box::new(List::Cons(2, Box::new(List::Nil))))),
    );
    println!("{:?}", my_recurrsive_list);

    // Deref

    let x = 5;
    let y = &x;

    println!("value of y => i32 (after derefecing with *) is {}", *y);

    let x = 5;
    let y = Box::new(x);
    println!("value of y => Box<i32> (after dereferencing) is {}", *y);

    //Custom Smart pointer
    let my_box = MyBox::new(4);
    println!("{:?}", my_box);
}

#[test]
fn test_my_box() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, *y); // *y here is same as calling *(y.deref())
}
