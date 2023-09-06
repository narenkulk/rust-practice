use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).unwrap_or_else(|err| {
        println!("Problem occurred while reading the contents {err}");
        process::exit(1);
    });
    println!(
        "Searching for string: '{}', in file contents:\n --------- \n {contents} \n --------- \n",
        config.query_string
    );

    for line in search(&config.query_string, &contents, config.ignore_case) {
        println!("Found: {line}");
    }
    // if contents.contains(config.query_string) {
    //     Ok("Found the matching string")
    // } else {
    //     process::exit(1)
    // }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    // we return the slices of
    // contents so it has to live as long as contents hence 'a for both
    // let mut results = Vec::new();
    if ignore_case {
        contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    } else {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }
    // for line in contents.lines() {
    //     if ignore_case {
    //         if line.to_lowercase().contains(&query.to_lowercase()) {
    //             results.push(line);
    //         }
    //     } else if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
}

pub struct Config {
    query_string: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        // if args.len() < 3 {
        //     return Err("Not enough arguments passed");
        // }
        args.next(); // the file path , we just ignore it
        let query_string = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query_string,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::search;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\n
            Rust:
fast, safe and productive!
            learn Rust today \n
            ";
        assert_eq!(
            vec!["fast, safe and productive!"],
            search(query, contents, false)
        );
    }

    #[test]
    fn search_case_insensitive() {
        let query = "rUsT";
        let contents = "\n
            Rust:
fast, safe and productive!
            learn Rust today \n
            ";
        assert_eq!(
            vec!["            Rust:", "            learn Rust today "],
            search(query, contents, true)
        );
    }
}
