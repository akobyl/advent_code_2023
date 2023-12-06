use std::cmp;
use std::collections::HashSet;

pub fn score_card(line: &str) -> usize {
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

    winners.intersection(&picks).count()
}
pub fn process(_input: &str) -> String {
    let pile_size: usize = _input.lines().count();
    let mut cardpile: Vec<u32> = vec![1; pile_size];

    for (i, line) in _input.lines().enumerate() {
        let score = score_card(line);
        let top = cmp::min(pile_size, i + score + 1);
        let multiplier = cardpile[i];

        cardpile
            .iter_mut()
            .take(top)
            .skip(i + 1)
            .for_each(|c| *c += multiplier);
    }

    let output: u32 = cardpile.iter().sum();
    output.to_string()
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
        assert_eq!("30", process(input))
    }

    #[test]
    fn test_score_card() {
        assert_eq!(
            4,
            score_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
        );
        assert_eq!(
            0,
            score_card("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36")
        );
    }
}