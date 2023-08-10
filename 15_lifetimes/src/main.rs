fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
 if x.len() > y.len(){
        x
    } else{
        y
    }
}


fn get_int_ref<'a>(int_1: &'a i32, int_2: &i32) -> &'a i32 {
    println!("{int_2}");
    &int_1
}
struct Foo<'a> {
    x: &'a str,
}

fn get_longest_string<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = "Hello";
    let string2 = "World";
    let longest_string;

    {
        let foo = Foo { x: string1 };
        longest_string = get_longest_string(foo.x, string2);
    }

    println!("Longest String: {}", longest_string);
}

// fn main() {
//     println!("longest string is: {}", longest("hello", "bye"));

//     let int_1 = 2;
//     let int_2 = 34;

//     let result = get_int_ref(&int_1, &int_2);
//     println!("{result}");
//     let result: &str;
//     let str1 = String::from("abcd");
// {
//         let str2 = String::from("xyz");
//         result = longest(str1.as_str(), str2.as_str());
//     }
//     println!("longest str is: {result}");

// }

