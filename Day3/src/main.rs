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

fn build_wire_instructions(vec: &Vec<&str>) -> Vec<(char, i32)> {
    let mut results = Vec::new();
    for x in vec {
        let s = x.chars().next();
        let ins = x.chars().next().map(|c| &x[c.len_utf8()..]);
        results.push((s.unwrap(), ins.unwrap().trim_end().parse::<i32>().unwrap()));
    }
    return results;
}

pub enum Direction {
    UNSET= 0,
    HORIZONTAL = 1,
    VERTICAL = 2,
}
struct Point {
    pub x:i32, 
    pub y:i32,
}

impl Point {
    pub fn new(_x:i32, _y:i32) -> Point {
        Point {
            x: _x,
            y: _y,
        }
    }
}

fn build_wire_segments(instr: Vec<(char, i32)>)-> Vec<(Point, Point, Direction)> {
    let mut segments : Vec<(Point, Point, Direction)> = Vec::new();
    let mut last_x = 0;
    let mut last_y = 0;
    for (a, b) in instr {
        let mut direction = Direction::UNSET;
        match a {
            'R' | 'L' => direction = Direction::HORIZONTAL,
            'U' | 'D' => direction = Direction::VERTICAL,
            _ => println!("wtf RLUD"),
        }
        let p1 = Point::new(last_x, last_y);
        let mut p2 = Point::new(0,0);
        match direction {
            Direction::HORIZONTAL => {
                last_x = b;
                p2 = Point::new(b, last_y)
            },
            Direction::VERTICAL => {
                last_y = b;
                p2 = Point::new(last_x, b)
            },
            _ => println!("wtf directions"),
        };
        &segments.push((p1, p2, direction));
    }
    return segments;
}

fn main() {
    let data = match parse_file("input.txt".chars().collect()) {
        Some(x) => x,
        None => String::new(),
    };
    //split the two wires from the input
    let vec = data.split('\n').collect::<Vec<&str>>();
    let _wire1_instr = build_wire_instructions(&vec[0].split(',').collect::<Vec<&str>>());
    let _wire2_instr = build_wire_instructions(&vec[1].split(',').collect::<Vec<&str>>());

    //build line segments for each data set
    let mut _wire_1_segments = build_wire_segments(_wire1_instr);
    let mut _wire_2_segments = build_wire_segments(_wire2_instr);
}
