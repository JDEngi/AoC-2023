use std::{fs::read_to_string, ops::Range};

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

struct SeedRange {
    start: u64,
    range: Range<u64>
}

impl SeedRange {
    fn from_range(start: u64, range: u64) -> SeedRange {
        SeedRange{start, range: start..start+range}
    }

    fn from_string(text: &str) -> SeedRange {
        let mut numbers = numbers_from_string(text);
        assert!(numbers.len() == 2, "SeedRange input length: {} != 2", numbers.len());

        let range = numbers.pop().unwrap();
        let start = numbers.pop().unwrap();

        SeedRange {start, range: start..start+range}
    }

    fn contains(&self, value: u64) -> bool {
        self.range.contains(&value)
    }
}

struct SeedBank {
    seeds: Vec<SeedRange>
}

impl SeedBank {
    fn new() -> SeedBank {
        SeedBank {seeds: vec![]}
    }    

    fn from_string(text: &str) -> SeedBank {
        let mut seed_bank = SeedBank::new();
        let numbers = numbers_from_string(text);

        for chunk in numbers.chunks(2) {
            if let [start, range] = chunk {
                seed_bank.sorted_insert(SeedRange::from_range(*start, *range));
            } else {
                panic!("Received a chunk that is not of size 2");
            }
        }

        seed_bank
    }

    fn sorted_insert(&mut self, seed_range: SeedRange) {
        let insertion_idx = match self.seeds.binary_search_by(|item| item.start.cmp(&seed_range.start)) {
            Ok(index) => index,
            Err(index) => index,
        };

        self.seeds.insert(insertion_idx, seed_range);
    }

    fn contains(&self, value: u64) -> bool {
        for seed_range in &self.seeds {
            if seed_range.contains(value) { 
                return true; 
            }
            else if value < seed_range.start {  // Short-circuit our lookup
                return false;
            }
        }

        return false;
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

    fn inv_remap(&self, value: u64) -> u64 {
        for mapping in &self.remaps {
            if value >= mapping.destination_start && value < mapping.destination_start + mapping.range {
                let source_value: u64 = (value as i128 - mapping.offset) as u64;
                return source_value;
            };

            // Disable short cicruits, because we need to keep a second sorted list for that.
            // Maybe do that later?
            // } else {
            //     value 
            // };
            // // If there are no more ranges that could match
            // if source_value < mapping.source_start {
            //     println!("Shortcut! {} < {}", source_value, mapping.source_start);
            //     return source_value;
            // }
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
    fn test_inv_remapper() {
        let mut test_remapper = Remapper::new();
        test_remapper.sorted_insert(Remap::from_string("50 98 2"));
        test_remapper.sorted_insert(Remap::from_string("52 50 48"));

        println!("Mappings are: ");
        for mapping in &test_remapper.remaps {
            println!("{:?}", mapping);
        }

        assert_eq!(test_remapper.remap(98), 50);
        assert_eq!(test_remapper.inv_remap(50), 98);
        assert_eq!(test_remapper.remap(50), 52);
        assert_eq!(test_remapper.inv_remap(52), 50);
        assert_eq!(test_remapper.inv_remap(40), 40);
    }

    #[test]
    fn test_seed_range_contains() {
        let mut seeds = SeedBank::new();
        seeds.sorted_insert(SeedRange::from_string("79 14"));
        seeds.sorted_insert(SeedRange::from_string("55 13"));

        assert!(seeds.contains(79));
        assert!(seeds.contains(92));
        assert!(!seeds.contains(93));
        assert!(!seeds.contains(94));
        assert!(seeds.contains(60));
        assert!(!seeds.contains(23));
    }
    
    #[test]
    fn iterated_inv_remapper() {
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

        let mut routing_table = vec![
            seed_to_soil, 
            soil_to_fertilizer, 
            fertilizer_to_water, 
            water_to_light, 
            light_to_temperature, 
            temperature_to_humidity, 
            humidity_to_location
        ];
        routing_table.reverse();

        // let seeds: Vec<u64> = vec![79, 14, 55, 13];
        let seeds = SeedBank{ seeds: vec![SeedRange::from_string("79 14"), SeedRange::from_string("55 13")]};
        
        let mut idx: u64 = 1;
        let lowest = loop {
            let inv_mapped = routing_table.iter().fold(idx, |cur_val, remapper| remapper.inv_remap(cur_val));
            println!("{} ===> {}", idx, inv_mapped);
            if  seeds.contains(inv_mapped) {
                break Some(idx);
            }

            idx += 1;
            if idx > 100 {
                break None;
            }
        };

        match lowest {
            Some(lowest) => assert!(lowest == 46, "Expected 46, got: {}", lowest),
            None => assert!(false, "No value was found")
        }
        // let result: Vec<u64> = seeds.iter().map(|seed| routing_table.iter().fold(*seed, |cur_val, remapper| remapper.remap(cur_val))).collect();

        // assert_eq!(expected, result);

    }
}

fn main() {
    let data = read_to_string("data/input.txt").unwrap();
    const SEEDS_HEADER: &str = "seeds:";

    // Get our seeds
    let first_line = data.lines().next().unwrap();
    let seeds = SeedBank::from_string(&first_line[SEEDS_HEADER.len()..]);

    // Build our routing table
    let mut inv_routing_table: Vec<Remapper> = Vec::new();
    for line in data.lines().skip(2) {
        let first_char = line.chars().next().unwrap_or('\n');
        if first_char.is_numeric() {
            if let Some(remapper) = inv_routing_table.last_mut() {
                remapper.sorted_insert(Remap::from_string(line));
            }
        }
        else if first_char == '\n' {}   // ignore
        else {
            println!("Now building the remapper for: {}", line);
            inv_routing_table.push(Remapper::new());
        }
    }
    inv_routing_table.reverse();

    let timer = std::time::Instant::now();
    let mut idx: u64 = 0;
    let lowest = loop {
        let inv_mapped = inv_routing_table.iter().fold(idx, |cur_val, remapper| remapper.inv_remap(cur_val));
        // println!("{} ===> {}", idx, inv_mapped);
        if  seeds.contains(inv_mapped) {
            break Some(idx);
        }

        idx += 1;
        if idx == u64::MAX {    // We're just going to assume that the answer isn't going to be u64::MAX
            break None;
        }
    };

    println!("Calculation time: {:.2?}", timer.elapsed());

    match lowest {
        Some(lowest) => println!("Lowest seed number is: {}", lowest),
        None => assert!(false, "No value was found")
    }

}
