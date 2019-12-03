
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

    let mut result = 0;
    let mut noun : usize = 0;
    let mut verb : usize = 0;
    let mut noun_found = false;
    
    while result != 19690720 {
        let mut vec : Vec<usize> = data.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        if !noun_found {
            noun += 1;
        }
        else {
            verb += 1;
        }
        
        vec[1] = noun;
        vec[2] = verb;
        let mut op_index : usize = 0;
    
        loop {
            if op_index+3 >= vec.len() {
                result = 0;
                break;
            }
            let op = &vec[op_index];
            if *op == 99 as usize {
                result = vec[0];
                if result > 19690720 {
                    noun_found = true;
                    noun -= 1;
                    result = 0;
                }
                break;
            }
            let c : usize = vec[op_index+3];
            match op {
                1 => {
                    vec[c] = vec[ (vec[op_index+1]) ] + vec[(vec[op_index+2])];
                }
                2 => {
                    vec[c] = vec[(vec[op_index+1])] * vec[(vec[op_index+2])];
                }
                _ => {
                    result = 0;
                    break;
                }
            }

            op_index += 4;
        }
    }
    println!("{} {}", result, 100 * noun + verb);
}
