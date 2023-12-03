use std::fs::read_to_string;

use anyhow::{Error, anyhow};

const RED_STONE_MAX: u8 = 12;
const GREEN_STONE_MAX: u8 = 13;
const BLUE_STONE_MAX: u8 = 14;

enum Stones {
    Red(u8),
    Blue(u8),
    Green(u8)
}

impl Stones {
    fn is_possible(&self) -> bool {
        match *self {
            Stones::Red(num) => num <= RED_STONE_MAX,
            Stones::Blue(num) => num <= BLUE_STONE_MAX,
            Stones::Green(num) => num <= GREEN_STONE_MAX,
        }
    }

    fn from_string(text: &str) -> Option<Stones> {
        let mut tokens = text.split_whitespace();

        let number_str = tokens.next()?;
        let number = number_str.parse::<u8>().unwrap();
        let color = tokens.last()?;

        match color {
            "red" => Some(Stones::Red(number)),
            "green" => Some(Stones::Green(number)),
            "blue" => Some(Stones::Blue(number)),
            _ => None,
        }
    }
}



// struct Subset(Vec<Stones>);

// impl IntoIterator for Subset {
//     type Item = Stones;
//     type IntoIter = std::vec::IntoIter<Stones>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

// // impl Subset {
// //     fn is_possible(&self) -> bool {
// //         for stone in self.0 {
// //             if !stone.is_possible() {return false};
// //         }
// //         true
// //     }

// //     fn new() -> Subset {
// //         Subset(Vec::new())
// //     }
// // }

fn subset_is_possible(subset: &Vec<Stones>) -> bool {
    // for stone in subset {
    //     if !stone.is_possible() {return false}
    // }
    // true

    subset.iter().all(|stone| stone.is_possible())
}


fn extract_stones_in_subset(text: &str) -> Vec<Stones> {
    let stone_strings = text.split_terminator(',');

    let mut stones_vec: Vec<Stones> = Vec::new();

    for string in stone_strings {
        if let Some(stone) = Stones::from_string(string) {
            stones_vec.push(stone);
        } else {
            println!("Failed to make stone")
        }
    }

    stones_vec
}

fn extract_game_number(text: &str) -> u32 {
    let mut tokens = text.split_whitespace();

    assert!(tokens.next().unwrap() == "Game");

    return tokens.last().unwrap().parse::<u32>().unwrap();
}

fn main() {
    
    let lines = read_to_string("../data/input.txt").unwrap();

    let mut game_sum: u32 = 0;

    for game in lines.lines() {
        // Split game header from subsets
        let mut line_split = game.split_terminator(':');

        let game_str = line_split.next().unwrap();
        let game_nr = extract_game_number(game_str);

        let subsets_string = line_split.last().unwrap();

        // Split up subsets
        let subset_strings = subsets_string.split(';');

        if subset_strings.into_iter().map(|subset| extract_stones_in_subset(subset)).all(|subset| subset_is_possible(&subset)) {
            game_sum += game_nr;
        } else {
            println!("{game_str} not possible")
        }
    }

    println!("Result output = {}", game_sum);
}
