use core::num;
use std::fs::read_to_string;


fn num_str_to_array(text: &str) -> Vec<u32> {
     text.split_ascii_whitespace()
         .map(|num| num.parse::<u32>().unwrap())
         .collect()
}


fn count_matching_elements(a: &mut Vec<u32>, b: &mut Vec<u32>) -> u32 {
    a.sort(); 
    b.sort();

    let mut a_idx = 0;
    let mut b_idx = 0;
    let mut count = 0;

    loop {
        let a_val = a.get(a_idx).unwrap();
        let b_val = b.get(b_idx).unwrap();

        if a_val == b_val { count += 1 }
        if a_val >= b_val { b_idx += 1 }
        if b_val >= a_val { a_idx += 1 }

        if a_idx == a.len() { break; }
        if b_idx == b.len() { break; }
    }

    count
}

fn count_to_score(count: u32) -> u32 {
    let base: u32 = 2;
    if count == 0 { 0 }
    else {
        base.pow(count as u32 - 1)
    }
}

fn extract_line_header(text: &str, delimiter: char) -> &str {
    &text[0..text.find(delimiter).unwrap()+1]
}

fn main() {
    let data = read_to_string("data/input.txt").unwrap();    // Unwrap, because if this fails, who cares about the rest

    let mut accumulator = 0;

    // iterate over lines
    for (idx, line) in data.lines().enumerate(){
        let idx = idx + 1;

        // Verify we receive the expected input (If we find out data is out of order, we can change this for a parser function)
        let header = extract_line_header(line, ':');

        // Don't need the header anymore, so we shadow line with the remainder of the line
        let line = &line[header.len()..];

        // Convert our strings to vector numbers
        let mut sections = line.split("|");
        let winning_str = sections.next().unwrap();
        let have_str = sections.next().unwrap();

        let mut winning_numbers = num_str_to_array(winning_str);
        let mut have_numbers = num_str_to_array(have_str);

        // The easy thing to do would be to check for_each number in a list if the other list contains it and then count those
        // However, this is a challenge game, and I want to take a slightly more challenging approach, so we implement a counting function

        winning_numbers.sort();
        have_numbers.sort();
        // println!("a:{winning_numbers:?}");
        // println!("b:{have_numbers:?}");
        
        let number_of_matches = count_matching_elements(&mut winning_numbers, &mut have_numbers);
        let score = count_to_score(number_of_matches);
        accumulator += score;


        println!("{} has {} matches, worth: {}", header, number_of_matches, score);
    }

    println!("Accumulator: {accumulator}");
}
