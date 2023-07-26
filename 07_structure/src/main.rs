#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }

    fn totally_detached_func(size: u32) -> u32 {
        size * size
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    struct User {
        is_active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        is_active: false,
        username: String::from("naren"),
        email: String::from("nk@noreply.com"),
        sign_in_count: 8,
    };

    let user_email = user1.email;
    println!(
        "user1's email is {user_email}, {}, {}, {}",
        user1.username, user1.is_active, user1.sign_in_count
    );

    let scale = 2;

    let rect1 = Rectangle {
        height: dbg!(30 * scale),
        width: 32,
    };

    dbg!(&rect1);
    println!("Rect is : {:#?}", rect1);
    println!("Area of a rectangle is : {}", area(&rect1));

    let rect2: Rectangle = Rectangle {
        height: 20,
        width: 21,
    };

    // using methods:
    println!("Area of a rectangle(using methods) is: {}", rect1.area());
    println!("Square: {:#?}", Rectangle::square(5));
    println!(
        "Totally detached function: {:?}",
        Rectangle::totally_detached_func(5)
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
