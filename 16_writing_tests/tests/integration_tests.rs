use writing_tests;

mod common;

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(x: i32) -> Guess {
        if x < 1 || x > 100 {
            panic!("Number should be between 1 and 100 but got {x}");
        } else {
            Guess { value: x }
        }
    }
}
#[test]
#[should_panic(expected = "should be between 1 and 100")]
fn guess_work_greater_than_100() {
    Guess::new(101);
}

#[test]
fn it_adds_two() {
    assert_eq!(4, writing_tests::add_two(2))
}
