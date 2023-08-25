#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red = 0;
        let mut blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => red += 1,
                ShirtColor::Blue => blue += 1,
            }
        }
        if red > blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let shirts = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let user_pref2 = None;

    println!(
        "user with {:?} preference got : {:?}",
        user_pref1,
        shirts.giveaway(user_pref1)
    );

    println!(
        "user with {:?} preference got : {:?}",
        user_pref2,
        shirts.giveaway(user_pref2)
    );

    // borrowing mutable variables
    //
    let mut list = vec![2, 4, 6];
    println!("Before modification by closure: {:?}", list);
    let mut modify_list = || {
        println!("list in closure: {:?}", list);
        list.push(8);
    };

    modify_list();
    println!("After modification by closure: {:?}", list);
}
