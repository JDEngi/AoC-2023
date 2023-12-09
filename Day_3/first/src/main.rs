use std::fs::read_to_string;

enum Tokens {
    Empty { start: u8, stop: u8 },
    Number { start: u8, stop: u8, value: u32 },
    Symbol { start: u8, stop: u8, value: char },
}

fn main() {
    let data = read_to_string("../data/test1").unwrap();
    let line_len = data.lines().next().unwrap().len();

    for line in data.lines() {
        
    }
}
