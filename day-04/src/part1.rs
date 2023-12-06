use std::collections::HashSet;

pub fn process(_input: &str) -> String {
    let mut score = 0;
    for line in _input.split('\n') {
        let split: Vec<&str> = line.split('|').collect();
        let winners: HashSet<u32> = split[0]
            .split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let picks: HashSet<u32> = split[1]
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let common_count = winners.intersection(&picks).count() as u32;
        if common_count > 0 {
            score += 2u32.pow(common_count - 1);
        }
    }
    score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", process(input))
    }
}
