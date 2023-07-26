use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter;

// print hello world

#[derive(Debug)]
enum Coins {
    Quarter(CANProvince),
    Cent,
    Dollar,
}

#[derive(Debug, EnumIter)]
enum CANProvince {
    Alberta = 1,
    Quebec = 2,
    Ontario = 3,
    Saskatchwan = 4,
}

struct Ipv4Addr {}

struct Ipv6Addr {}

enum IPAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// Loop through CANProvince enum variants

fn main() {
    match_and_operate(Some(5));
    for province in CANProvince::iter() {
        match_coins(Coins::Quarter(province))
    }
}

fn match_coins(coin: Coins) {
    // match coin {
    //     Coins::Quarter(province) => {
    //         println!("Your province: {:#?}", province);
    //     }
    //     _ => {}
    // }

    if let Coins::Quarter(province) = coin {
        println!("province in if let is: {:?}", province);
    } else {
        println!("Not a quarter")
    }
}

fn match_and_operate(x: Option<u32>) {
    match x {
        Some(x) => {
            let x = x + 1; // here you can bind x variable and use it to perform operations on 'T' of Option<T>
            println!("Some - plus one: {} ", x);
        }
        None => {
            println!("None");
        }
    }
}
