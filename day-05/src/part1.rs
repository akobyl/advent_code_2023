struct Range {
    source_start: u64,
    source_end: u64,
    dest_start: u64,
}

impl Range {
    fn new(slice: &[u64]) -> Range {
        if slice.len() < 3 {
            panic!("need 3 u64's");
        }

        Range {
            dest_start: slice[0],
            source_start: slice[1],
            source_end: slice[1] + slice[2] - 1,
        }
    }

    fn process_val(&self, value: u64) -> Option<u64> {
        if value >= self.source_start && value <= self.source_end {
            return Some(self.dest_start + value - self.source_start);
        }
        None
    }
}

fn process_map(map: &[Range], value: u64) -> u64 {
    let found = map.iter().find_map(|r| r.process_val(value));

    match found {
        Some(found) => found,
        None => value,
    }
}

fn generate_map(map_str: &Vec<&str>) -> Vec<Range> {
    let mut map: Vec<Range> = Vec::new();

    for line in map_str.iter().skip(1) {
        let nums: Vec<u64> = line
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect();

        map.push(Range::new(&nums));
    }
    map
}

pub fn process(input: &str) -> String {
    let seeds: Vec<u64> = input
        .lines()
        .take(2)
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect();

    let lines: Vec<&str> = input.lines().skip(2).collect();

    let mut maps: Vec<Vec<&str>> = Vec::new();
    let mut current_map: Vec<&str> = Vec::new();

    for item in lines {
        if item.is_empty() {
            if !current_map.is_empty() {
                maps.push(current_map);
                current_map = Vec::new();
            }
        } else {
            current_map.push(item);
        }
    }

    if !current_map.is_empty() {
        maps.push(current_map);
    }

    let generated_maps: Vec<Vec<Range>> = maps.iter().map(generate_map).collect();

    let location: u64 = seeds
        .iter()
        .map(|&seed| {
            generated_maps
                .iter()
                .fold(seed, |value, map| process_map(map, value))
        })
        .min()
        .unwrap();

    location.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "seeds: 79 14 55 13\n\
                    \n\
                    seed-to-soil map:\n\
                    50 98 2\n\
                    52 50 48\n\
                    \n\
                    soil-to-fertilizer map:\n\
                    0 15 37\n\
                    37 52 2\n\
                    39 0 15\n\
                    \n\
                    fertilizer-to-water map:\n\
                    49 53 8\n\
                    0 11 42\n\
                    42 0 7\n\
                    57 7 4\n\
                    \n\
                    water-to-light map:\n\
                    88 18 7\n\
                    18 25 70\n\
                    \n\
                    light-to-temperature map:\n\
                    45 77 23\n\
                    81 45 19\n\
                    68 64 13\n\
                    \n\
                    temperature-to-humidity map:\n\
                    0 69 1\n\
                    1 0 69\n\
                    \n\
                    humidity-to-location map:\n\
                    60 56 37\n\
                    56 93 4";
        assert_eq!("35", process(input))
    }
}
