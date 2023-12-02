use std::fs::read_to_string;
pub use regex::Regex;


enum Numbers {
    String(String),
    Digit(char)
}

impl Numbers {
    fn from_string(text: &str) -> Numbers {
        if text.len() == 1 {
            Numbers::Digit(text.chars().next().unwrap())
        } else {
            Numbers::String(text.to_string())
        }
    }

    fn to_digit(self) -> u8 {
        let digit = match (self) {
            Numbers::String(text) => {
                if text == "one" {'1'}
                else if text == "two" {'2'}
                else if text == "three" {'3'}
                else if text == "four" {'4'}
                else if text == "five" {'5'}
                else if text == "six" {'6'}
                else if text == "seven" {'7'}
                else if text == "eight" {'8'}
                else if text == "nine" {'9'}
                else {'0'}
            },
            Numbers::Digit(c) => c,
        };
        
        digit.to_digit(RADIX).unwrap().try_into().unwrap()
    }
}

static NUMBERS_RE: &'static str = r"([0-9]|one|two|three|four|five|six|seven|eight|nine|zero)";

#[cfg(test)]
mod tests {
    use regex::Regex;

    static NUMBERS_RE: &'static str = r"([0-9]|one|two|three|four|five|six|seven|eight|nine|zero)";

    fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }

    #[test]
    fn test_regex_number_pattern()
    {
        let number_re = Regex::new(NUMBERS_RE).unwrap();

        let numbers = "1234567890onetwothreefourfivesixseveneightninezero";
        
        let want = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero"];
        let get: Vec<&str> = number_re.find_iter(numbers).map(|cap| cap.as_str()).collect();

        for (a, b) in get.iter().zip(want.iter()) {
            if a != b {
                println!("mismatch: {} != {}", a, b);
                assert!(false);
            }
        }
        assert!(get.len() == want.len());
        assert!(do_vecs_match(&want, &get));
    }

    #[test]
    fn test_appending_empty_vec() {
        let mut a: Vec<u8> = vec![1, 2, 3];
        let mut b: Vec<u8> = Vec::new();

        println!("a: {a:?}, b: {b:?}");

        a.append(&mut b);

        println!("a: {a:?}");
    }
}

const RADIX: u32 = 10;

fn extract_output(numbers: &Vec<u8>) -> u8 {
    if numbers.is_empty(){
        0
    } else {
        (numbers.first().unwrap() * 10) + numbers.last().unwrap()
    }

}

// This function doesn't work, because it can't find overlapping number strings
// Leaving this here for posterity
// fn extract_numbers(text: &str, reg_exp: &regex::Regex) -> Vec<u8>{
//     reg_exp.find_iter(text)
//             .map(|cap| cap.as_str())
//             .map(|text| Numbers::from_string(text).to_digit()).collect()
// }

fn recursive_regex_extract(text: &str, reg_exp: &regex::Regex) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    if let Some(value) = reg_exp.find(text) {
        result.push(Numbers::from_string(value.as_str()).to_digit());
        result.append(&mut recursive_regex_extract(&text[value.start()+1..], reg_exp));
        return result;
    } else {
        return result;
    }
}

fn main() {
    // Read file as string input
    let file_data = read_to_string("../data/input2").unwrap();

    let numbers_regex = Regex::new(NUMBERS_RE).unwrap();

    let mut accumulator: u32 = 0;
    let mut count = 0;
    for line in file_data.lines() {
        let numbers = recursive_regex_extract(line, &numbers_regex);
        let output = extract_output(&numbers) as u32;

        println!("input: {line}, numbers: {numbers:?}, result: {output}");

        accumulator += output;
        count += 1;
    }

    println!("The adjust output is: {accumulator}. Calculated by summing: {count}");
}
