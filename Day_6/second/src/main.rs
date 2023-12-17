use core::time;
use std::fs::read_to_string;

fn numbers_from_string(text: &str) -> Vec<u32> {
    text.split_ascii_whitespace().map(|number| number.parse::<u32>().expect("Number larger than u32 max")).collect()
}

fn get_number_ignore_whitespace(text: &str) -> u64 {
    let fixed_str = text.replace(" ", "");
    return fixed_str.parse::<u64>().unwrap();
}

fn ceil_div(a: i128, b:i128) -> i128 {
    a/b + (a % b).signum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ceil_div() {
        assert!(ceil_div(9, 7) == 2, "9/7 ==> 2");
    }

    #[test]
    fn test_fix_kerning() {
        assert!(get_number_ignore_whitespace("     7  15   30") == 71530);
        assert!(get_number_ignore_whitespace(" 9  40  200") == 940200);
    }

    #[test]
    fn check_solutions() {
        let time = 7;
        let distance = 9;
        let expected_wins = 4;
        let possibly = 2..7;
        let winning = 
            possibly
                .map(|num| (time-num) * num)
                .inspect(|&val| println!("Possible distance: {val} {} {distance}",if val > distance {">"} else {"<="} ))
                .filter(|&dist| dist > distance)
                .count();

        assert!(winning == expected_wins, "Test did not yield expected wins: {} != {}", winning, expected_wins);
    }
}


fn main() {
    let mut data = read_to_string("data/input.txt").unwrap();

    let time_str = data.lines().next().unwrap();
    let time = get_number_ignore_whitespace(&time_str["Time: ".len()..]);
    let distance_str = data.lines().last().unwrap();
    let goal = get_number_ignore_whitespace(&distance_str["Distance: ".len()..]);

    println!("Race conditions: Time: {time}, distance: {goal}");

    // let winning = 
    //     possible_range
    //         .map(|num| (time-num) * num)
    //         .inspect(|&val| println!("Possible distance: {val} {} {distance}",if val > distance {">"} else {"<="} ))
    //         .filter(|&dist| dist > distance)
    //         .count();
    
    let mut winning_count: u32 = 0;
    let minimum_speed = ceil_div(goal.try_into().unwrap(), time.try_into().unwrap()) as u64;
    println!("minimum_speed: {minimum_speed}");
    for speed in minimum_speed..time {
        let distance = (time - speed) * speed;

        if distance > goal { winning_count += 1}
        // else {
        //     println!("[{speed}]{distance} <= {goal}");
        //     break;
        // }
        // else { break;}
    }
        


    println!("The product of wins is: {}", winning_count);
}
