// usage of iter().map()
fn main() {
    let v1 = vec![1, 2, 3];
    let new_v1: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", new_v1);
}
