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
