use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    println!("scores dict: {:?}", scores);
    // println!("scores dict: {:?}", scores.entry(String::from("Blue")));

    let mut words = HashMap::new();
    let text = String::from("Wonderful world , Hello world world!");
    for word in text.split(|c: char| !c.is_alphanumeric()) {
        let count = words.entry(word).or_insert(0);
        *count += 1;
    }
    println!("words dict: {:?}", words);

    // Nested hashmaps:

    #[derive(Serialize, Deserialize, Debug)]
    struct Entry {
        file: String,
        directory: HashMap<String, Entry>,
    }

    let mut map: HashMap<String, HashMap<String, Entry>> = HashMap::new();
    let mut inner_map = HashMap::new();

    inner_map.insert(
        String::from("Directory-1"),
        Entry {
            file: String::from("File-1"),
            directory: HashMap::new(),
        },
    );

    map.insert(String::from("C Drive"), inner_map);
    let json = json!(map);
    // let formatter = PrettyFormatter::with_indent(b"\t");
    let pretty_json = serde_json::to_string_pretty(&json).unwrap();

    println!("{:#?}", map);
    println!("{}", pretty_json);
//     {
//     "C Drive": {
//         "Directory-1": Entry {
//             file: "File-1",
//             directory: {},
//         },
//     },
// }
// {
//   "C Drive": {
//     "Directory-1": {
//       "directory": {},
//       "file": "File-1"
//     }
//   }
// }
}
