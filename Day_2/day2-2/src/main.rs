use std::{fs::read_to_string, cmp::max};

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

fn subset_is_possible(subset: &Vec<Stones>) -> bool {
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

fn calculate_power(subsets: &Vec<Vec<Stones>>) -> u32 {
    let mut max_red: u8 = 0;
    let mut max_green: u8 = 0;
    let mut max_blue: u8 = 0;

    for subset in subsets {
        for stone in subset {
            match stone {
                Stones::Red(val) => {max_red = max(*val, max_red)},
                Stones::Blue(val) => {max_blue = max(*val, max_blue)},
                Stones::Green(val) => {max_green = max(*val, max_green)},
            }
        }
    }

    max_red as u32 * max_green as u32 * max_blue as u32
}

fn main() {
    
    let lines = read_to_string("../data/input.txt").unwrap();

    let mut game_sum: u32 = 0;
    let mut game_power: u32 = 0;

    for game in lines.lines() {
        // Split game header from subsets
        let mut line_split = game.split_terminator(':');

        let game_str = line_split.next().unwrap();
        let game_nr = extract_game_number(game_str);

        let subsets_string = line_split.last().unwrap();

        // Split up subsets
        let subset_strings = subsets_string.split(';');

        println!("{game}");
        let subsets: Vec<Vec<Stones>> = subset_strings.into_iter().map(|subset| extract_stones_in_subset(subset)).collect();
        // if subsets.iter().all(|subset| subset_is_possible(subset)) {


        //     println!("Power in game {game_nr} = {}", calculate_power(&subsets));
        // } else {
        //     println!("{game_str} is not possible");
        // }

        game_power += calculate_power(&subsets);
        game_sum += game_nr;

    }

    println!("Result output = {}", game_sum);
    println!("Result power = {game_power}");
}
