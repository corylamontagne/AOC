use std::char;
extern crate fancy_regex;
use fancy_regex::Regex;

fn in_order(buf: &String) ->bool {
    let data: Vec<_> = buf.chars().map(|n| n.to_digit(10).unwrap()).collect();
    for i in 0..data.len()-1 {
        if &data[i] > &data[i+1]{
            return false;
        }
    }
    true
}

fn main() {
    let mut res = 0;
    let mut start = 357253;
    let end = 892942;
    while start <= end {
        let s = start.to_string();
        if in_order(&s) {
            let reg_ex = Regex::new(r"(?:^|(.)(?!\1))(\d)\2(?!\2)").unwrap();
            if reg_ex.is_match(&s).unwrap() { 
                res+=1; 
            }
        }
        start += 1;
    }
    println!("{}", res);

}
