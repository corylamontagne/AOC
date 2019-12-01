
use std::fs::File;
use std::io::prelude::*;

fn parse_file(file: String) -> Option<String> {
    let mut content = String::new();
    match File::open(file.clone()) {
        // The file is open (no error).
        Ok(mut file) => {
            // Read contents to string
            file.read_to_string(&mut content).unwrap();
        },
        // Error handling.
        Err(error) => {
            println!("Error opening file {}: {}", file, error);
            return None;
        },
    }
    Some(content)
}

fn calc_fuel(x: i64) -> i64 {
    (x / 3) - 2
}

fn main() {
    let input_data = parse_file("input.txt".chars().collect()); 

    let data = match input_data {
        Some(x) => x,
        None => String::new(),
    };

    let vec = data.split_whitespace().collect::<Vec<&str>>();

    let mut result = 0;
    for x in vec {
        let mut y = calc_fuel(x.trim_end().parse::<i64>().unwrap()); 
        while y > 0  {
            result += y;
            y = calc_fuel(y);
        }
    }

    println!("{}", result);
}
