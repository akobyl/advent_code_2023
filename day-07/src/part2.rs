use core::fmt;
use std::collections::HashMap;

#[derive(Debug, Ord, PartialEq, PartialOrd, Eq)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl HandType {
    fn from_str(s: &str) -> Self {
        let jokerless = s.chars().filter(|c| *c != 'J').collect::<String>();

        if jokerless.is_empty() {
            return HandType::FiveOfAKind;
        }

        let counts = jokerless.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        let mut count_vec: Vec<_> = counts.values().copied().collect();
        count_vec.sort_unstable_by(|a, b| b.cmp(a));

        if let Some(first) = count_vec.first_mut() {
            *first += s.chars().filter(|c| *c == 'J').count() as u32;
        }

        match count_vec.as_slice() {
            [5, ..] => HandType::FiveOfAKind,
            [4, ..] => HandType::FourOfAKind,
            [3, 2, ..] => HandType::FullHouse,
            [3, ..] => HandType::ThreeOfAKind,
            [2, 2, ..] => HandType::TwoPair,
            [2, ..] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug, Ord, PartialEq, PartialOrd, Eq)]
enum CardValue {
    Num(u8),
    T,
    J,
    Q,
    K,
    A,
}

impl CardValue {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(CardValue::A),
            'K' => Some(CardValue::K),
            'Q' => Some(CardValue::Q),
            'J' => Some(CardValue::J),
            'T' => Some(CardValue::T),
            _ => c.to_digit(10).map(|n| CardValue::Num(n as u8)),
        }
    }

    fn value(&self) -> u32 {
        match self {
            CardValue::J => 0x01, // J is now the lowest value
            CardValue::Num(n) if *n <= 9 => *n as u32,
            CardValue::T => 0x0a,
            CardValue::Q => 0x0c,
            CardValue::K => 0x0d,
            CardValue::A => 0x0e,
            _ => panic!("should never happen"),
        }
    }
}

#[derive(Eq)]
struct Hand {
    hand_type: HandType,
    bid: u32,
    score: u32,
    string: String,
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Hand {{ string: {}, score: {:04x}, hand_type: {:?}, bid: {}}}",
            self.string, self.score, self.hand_type, self.bid
        )
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.score == other.score
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then_with(|| self.score.cmp(&other.score))
    }
}

fn score_hand(hand_str: &str) -> u32 {
    hand_str
        .chars()
        .map(|c| CardValue::from_char(c).unwrap())
        .map(|cv| cv.value())
        .fold(0u32, |acc, card_value| (acc << 4) | card_value)
}

pub fn process(input: &str) -> String {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let (hand_str, bid_str) = line.split_once(' ').unwrap();
            Hand {
                hand_type: HandType::from_str(hand_str),
                bid: bid_str.parse().unwrap(),
                score: score_hand(hand_str),
                string: hand_str.to_string(),
            }
        })
        .collect();
    hands.sort();

    let output: u32 = hands
        .iter()
        .enumerate()
        .fold(0u32, |acc, (i, hand)| acc + hand.bid * (i as u32 + 1));
    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "32T3K 765\n\
                           T55J5 684\n\
                           KK677 28\n\
                           KTJJT 220\n\
                           QQQJA 483";
        assert_eq!("5905", process(input))
    }

    #[test]
    fn test_scoring() {
        assert_eq!(score_hand("TJQKA"), 0xa1cde);
    }

    #[test]
    fn test_hand_type_from_str() {
        assert_eq!(HandType::from_str("32T3J"), HandType::ThreeOfAKind);
        assert_eq!(HandType::from_str("T55J5"), HandType::FourOfAKind);
        assert_eq!(HandType::from_str("KTJJT"), HandType::FourOfAKind);
        assert_eq!(HandType::from_str("QQQJA"), HandType::FourOfAKind);
        assert_eq!(HandType::from_str("JJJJJ"), HandType::FiveOfAKind);
    }
}
