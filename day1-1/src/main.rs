use std::{fs::File, fs::read_to_string, result};

const RADIX: u32 = 10;

fn extract_number(text: &str) -> i32 {
    let mut number_str = String::new();

    text.chars().filter(|c| c.is_numeric()).map(|c| number_str.push(c)).collect::<Vec<_>>();

    let first = number_str.chars().nth(0).unwrap();
    let last = number_str.chars().last().unwrap();

    let result = first.to_digit(RADIX).unwrap() * 10 + last.to_digit(RADIX).unwrap();

    println!("{} => {}", text, result);

    result as i32
}

fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!");

    // read file line-by-line
    let file_data = read_to_string("data/input")?;

    // collect the numbers
    let result = file_data.lines().map(|line| extract_number(line));

    let mut accumulator = 0;
    let mut count = 0;

    for number in result {
        println!("{}", number);
        accumulator += number;
        count += 1;
    }

    println!("Result = {}", accumulator);
    println!("Derived from {} numbers", count);

    Ok(())
}
