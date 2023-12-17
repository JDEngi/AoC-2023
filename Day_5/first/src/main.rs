use std::fs::read_to_string;

fn numbers_from_string(text: &str) -> Vec<u64> {
    text.split_ascii_whitespace().map(|number| number.parse::<u64>().expect("Number larger than u32 max")).collect()
}

#[derive(Debug)]
struct Remap {
    destination_start: u64,
    source_start: u64,
    range: u64,
    offset: i128     // calculated at creation time to simplify further calculationx
}

impl Remap {
    fn from_string(text: &str) -> Remap {
        let mut numbers = numbers_from_string(text);
        assert!(numbers.len() == 3, "Invalid string input for creating Remap");

        let range = numbers.pop().unwrap();
        let source_start = numbers.pop().unwrap();
        let destination_start = numbers.pop().unwrap();
        let offset: i128 = destination_start as i128 - source_start as i128;

        Remap {destination_start, source_start, range, offset}
    }   
}


struct Remapper {
    remaps: Vec<Remap>
}

impl Remapper {
    fn new() -> Remapper {
        Remapper {remaps: vec![]}
    }

    fn remap(&self, value: u64) -> u64 {
        for mapping in &self.remaps {
            if value >= mapping.source_start && value < mapping.source_start + mapping.range {
                let new_value = value as i128 + mapping.offset;
                return new_value.try_into().expect("value of out range");
            }
            // If there are no more ranges that could match
            else if value < mapping.source_start {
                return value;
            }
        }

        return value;
    }

    fn sorted_insert(&mut self, mapping: Remap) {
        let insertion_idx = match self.remaps.binary_search_by(|item| item.source_start.cmp(&mapping.source_start)) {
            Ok(index) => index,
            Err(index) => index,
        };

        self.remaps.insert(insertion_idx, mapping);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remapper() {
        let mut test_remapper = Remapper::new();
        test_remapper.sorted_insert(Remap::from_string("50 98 2"));
        test_remapper.sorted_insert(Remap::from_string("52 50 48"));

        println!("Mappings are: ");
        for mapping in &test_remapper.remaps {
            println!("{:?}", mapping);
        }

        assert_eq!(test_remapper.remap(98), 50);
        assert_eq!(test_remapper.remap(50), 52);
    }

    #[test]
    fn iterated_remapper() {
        let mut seed_to_soil = Remapper::new();
        seed_to_soil.sorted_insert(Remap::from_string("50 98 2"));
        seed_to_soil.sorted_insert(Remap::from_string("52 50 48"));

        let mut soil_to_fertilizer = Remapper::new();
        soil_to_fertilizer.sorted_insert(Remap::from_string("0 15 37"));
        soil_to_fertilizer.sorted_insert(Remap::from_string("37 52 2"));
        soil_to_fertilizer.sorted_insert(Remap::from_string("39 0 15"));

        let mut fertilizer_to_water = Remapper::new();
        fertilizer_to_water.sorted_insert(Remap::from_string("49 53 8"));
        fertilizer_to_water.sorted_insert(Remap::from_string("0 11 42"));
        fertilizer_to_water.sorted_insert(Remap::from_string("42 0 7"));
        fertilizer_to_water.sorted_insert(Remap::from_string("57 7 4"));

        let mut water_to_light = Remapper::new();
        water_to_light.sorted_insert(Remap::from_string("88 18 7"));
        water_to_light.sorted_insert(Remap::from_string("18 25 70"));

        let mut light_to_temperature = Remapper::new();
        light_to_temperature.sorted_insert(Remap::from_string("45 77 23"));
        light_to_temperature.sorted_insert(Remap::from_string("81 45 19"));
        light_to_temperature.sorted_insert(Remap::from_string("68 64 13"));

        let mut temperature_to_humidity = Remapper::new();
        temperature_to_humidity.sorted_insert(Remap::from_string("0 69 1"));
        temperature_to_humidity.sorted_insert(Remap::from_string("1 0 69"));

        let mut humidity_to_location = Remapper::new();
        humidity_to_location.sorted_insert(Remap::from_string("60 56 37"));
        humidity_to_location.sorted_insert(Remap::from_string("56 93 4"));

        let routing_table = vec![
            seed_to_soil, 
            soil_to_fertilizer, 
            fertilizer_to_water, 
            water_to_light, 
            light_to_temperature, 
            temperature_to_humidity, 
            humidity_to_location
        ];
        let seeds: Vec<u64> = vec![79, 14, 55, 13];
        let expected: Vec<u64> = vec![82, 43, 86, 35];

        let result: Vec<u64> = seeds.iter().map(|seed| routing_table.iter().fold(*seed, |cur_val, remapper| remapper.remap(cur_val))).collect();

        assert_eq!(expected, result);

    }
}

fn main() {
    

    let data = read_to_string("data/input.txt").unwrap();
    const SEEDS_HEADER: &str = "seeds:";

    // Get our seeds
    let first_line = data.lines().next().unwrap();
    let seeds = numbers_from_string(&first_line[SEEDS_HEADER.len()..]);

    let timer = std::time::Instant::now();
    // Build our routing table
    let mut routing_table: Vec<Remapper> = Vec::new();
    for line in data.lines().skip(2) {
        let first_char = line.chars().next().unwrap_or('\n');
        if first_char.is_numeric() {
            if let Some(remapper) = routing_table.last_mut() {
                remapper.sorted_insert(Remap::from_string(line));
            }
        }
        else if first_char == '\n' {}   // ignore
        else {
            println!("Now building the remapper for: {}", line);
            routing_table.push(Remapper::new());
        }
    }
    println!("router parsing: {:.2?}", timer.elapsed());

    let processing_timer = std::time::Instant::now();
    let locations: Vec<u64> = seeds.iter().map(|seed| routing_table.iter().fold(*seed, |cur_val, remapper| remapper.remap(cur_val))).collect();

    println!("Mapping pargins: {:.2?}", processing_timer.elapsed());

    println!("Results are (as expected): {:?}", locations);
    println!("Lowest location number is: {}", locations.iter().min().unwrap());

}
