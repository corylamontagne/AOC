use std::char;

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
            let mut found = true;
            for i in 0..9 {
                let c: Vec<&str> = s.matches( char::from_digit(i as u32, 10).unwrap() ).collect();
                let l = c.len();
                if l > 1 && l % 2 == 1 {
                    found = false;
                    println!("{}", s);
                    break;
                }
            }
            if found {res += 1;}
        }
        start += 1;
    }
    println!("{}", res);

}
