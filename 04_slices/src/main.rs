fn main() {
    println!("Hello, world!");
    let mut sentence = String::from("apples,oranges,and,bananas");
    let word = first_word(&mut sentence);
    println!("first word is: {word}");
    sentence.clear();
    println!("sentence: {sentence}");
    // println!("first word is: {word}");
}

fn first_word(whole_string: &mut String) -> &str {
    let arr_bytes = whole_string.as_bytes();
    for (i, &item) in arr_bytes.iter().enumerate() {
        if item == b' ' {
            return &whole_string[..i];
        }
    }
    return &whole_string[..];
}
