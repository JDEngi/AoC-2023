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

fn extract_line_header(text: &str, delimiter: char) -> &str {
    &text[0..text.find(delimiter).unwrap()+1]
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_input() {
        let data = read_to_string("data/test1").unwrap();
    }
}

fn main() {
    let data = read_to_string("data/input.txt").unwrap();    // Unwrap, because if this fails, who cares about the rest

    let mut list_of_matches: Vec<u32> = Vec::new();

    // iterate over lines
    for line in data.lines(){
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

        winning_numbers.sort();
        have_numbers.sort();
        let number_of_matches = count_matching_elements(&mut winning_numbers, &mut have_numbers);

        list_of_matches.push(number_of_matches);

        println!("{} has {} matches", header, number_of_matches);
    }

    let mut list_of_card_counts: Vec<u32> = vec![1; list_of_matches.len()];
    
    for (idx, match_count) in list_of_matches.clone().into_iter().enumerate() {
        if match_count == 0 {continue;}
        let cur_count = *list_of_card_counts.get(idx).unwrap();

        for next_count in &mut list_of_card_counts[idx+1..(idx+1+match_count as usize)] {
            *next_count += cur_count;
        }

        println!("{:?}", list_of_card_counts);
    }

    let total_cards = list_of_card_counts.into_iter().fold(0, |a, b| a+b);

    println!("Total number of cards: {}", total_cards);
}
