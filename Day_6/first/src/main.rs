use std::fs::read_to_string;

fn numbers_from_string(text: &str) -> Vec<u32> {
    text.split_ascii_whitespace().map(|number| number.parse::<u32>().expect("Number larger than u32 max")).collect()
}


fn ceil_div(a: i32, b:i32) -> i32 {
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

    let times_str = data.lines().next().unwrap();
    let times = numbers_from_string(&times_str["Time: ".len()..]);
    let distances_str = data.lines().last().unwrap();
    let distances = numbers_from_string(&distances_str["Distance: ".len()..]);

    let mut winning_moves: Vec<u32> = Vec::new();

    for (&time, &distance) in times.iter().zip(distances.iter()) {
        let possible_range = ceil_div(distance.try_into().unwrap(), time.try_into().unwrap()) as u32..time;

        let winning = 
            possible_range
                .map(|num| (time-num) * num)
                .inspect(|&val| println!("Possible distance: {val} {} {distance}",if val > distance {">"} else {"<="} ))
                .filter(|&dist| dist > distance)
                .count();
        
        winning_moves.push(winning as u32);
    }

    println!("The product of wins is: {}", winning_moves.iter().fold(1, |a, b| a * b));
}
