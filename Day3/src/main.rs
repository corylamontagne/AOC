use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

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

fn main() {
    let data = match parse_file("input.txt".chars().collect()) {
        Some(x) => x,
        None => String::new(),
    };
    //split the two wires from the input
    let vec = data.split('\n').collect::<Vec<&str>>();

    //get the vector of values for each wire
    let _wire1 = vec[0].split('\n').collect::<Vec<&str>>();
    let _wire2 = vec[1].split('\n').collect::<Vec<&str>>();

    
}
