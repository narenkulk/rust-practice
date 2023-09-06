#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn matching_shoes() {
    let shoes_list = vec![
        Shoe {
            size: 10,
            style: String::from("sneakers"),
        },
        Shoe {
            size: 13,
            style: String::from("boots"),
        },
        Shoe {
            size: 15,
            style: String::from("sandals"),
        },
    ];

    let matched_shoes = shoes_in_size(shoes_list, 10);

    assert_eq!(
        matched_shoes,
        vec![Shoe {
            size: 10,
            style: String::from("sneakers")
        }]
    );
}

#[test]
fn iterators_demonstration() {
    let v = vec![1, 2, 3];
    let mut iter_v = v.iter();
    assert_eq!(iter_v.next(), Some(&1));
    assert_eq!(iter_v.next(), Some(&2));
    assert_eq!(iter_v.next(), Some(&3));
    assert_eq!(iter_v.next(), None);
}
#[test]
fn into_iter_demo() {
    let v = vec![1, 2, 3];
    let mut iter_v = v.into_iter();
    assert_eq!(iter_v.next(), Some(1));
    assert_eq!(iter_v.next(), Some(2));
    assert_eq!(iter_v.next(), Some(3));
    assert_eq!(iter_v.next(), None);
}
#[test]
fn iter_mut_demo() {
    let mut v = vec![1, 2, 3];
    let mut iter_v = v.iter_mut();
    assert_eq!(iter_v.next(), Some(&mut 1));
    assert_eq!(iter_v.next(), Some(&mut 2));
    assert_eq!(iter_v.next(), Some(&mut 3));
    assert_eq!(iter_v.next(), None);
}
#[test]
fn iterators_sum() {
    let v = vec![1, 2, 3];
    let total: i32 = v.iter().sum();
    assert_eq!(total, 6);
}
