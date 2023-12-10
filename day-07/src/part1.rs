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
        let counts = s.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        let mut count_vec: Vec<_> = counts.values().copied().collect();
        count_vec.sort_unstable_by(|a, b| b.cmp(a));

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
            CardValue::Num(n) if *n <= 9 => *n as u32,
            CardValue::T => 0x0a,
            CardValue::J => 0x0b,
            CardValue::Q => 0x0c,
            CardValue::K => 0x0d,
            CardValue::A => 0x0e,
            _ => panic!("should never happen"),
        }
    }
}

#[derive(Debug, Eq)]
struct Hand {
    hand_type: HandType,
    bid: u32,
    score: u32,
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
    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let (hand_str, bid_str) = line.split_once(' ').unwrap();
        let bid: u32 = bid_str.parse().unwrap();
        let hand_type = HandType::from_str(hand_str);
        let hand = Hand {
            hand_type,
            bid,
            score: score_hand(hand_str),
        };
        hands.push(hand);
    }
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
        assert_eq!("6440", process(input))
    }

    #[test]
    fn test_scoring() {
        assert_eq!(score_hand("23456"), 0x23456);
        assert_eq!(score_hand("TJQKA"), 0xabcde);
        assert!(score_hand("KK677") > score_hand("KTJJT"));
    }

    #[test]
    fn test_hand_type_from_str() {
        assert_eq!(HandType::from_str("AAAAA"), HandType::FiveOfAKind);
        assert_eq!(HandType::from_str("A4AAA"), HandType::FourOfAKind);
        assert_eq!(HandType::from_str("A444A"), HandType::FullHouse);
        assert_eq!(HandType::from_str("KKAAQ"), HandType::TwoPair);
        assert_eq!(HandType::from_str("23456"), HandType::HighCard);
        assert_eq!(HandType::from_str("23456"), HandType::HighCard);

        assert_eq!(HandType::from_str("32T3K"), HandType::OnePair);
        assert_eq!(HandType::from_str("T55J5"), HandType::ThreeOfAKind);
        assert_eq!(HandType::from_str("KK677"), HandType::TwoPair);
        assert_eq!(HandType::from_str("KTJJT"), HandType::TwoPair);
        assert_eq!(HandType::from_str("QQQJA"), HandType::ThreeOfAKind);
    }

    #[test]
    fn test_card_value() {
        assert_eq!(CardValue::from_char('A'), Some(CardValue::A));
        assert_eq!(CardValue::from_char('K'), Some(CardValue::K));
        assert_eq!(CardValue::from_char('J'), Some(CardValue::J));
        assert_eq!(CardValue::from_char('Q'), Some(CardValue::Q));
        assert_eq!(CardValue::from_char('T'), Some(CardValue::T));
        assert_eq!(CardValue::from_char('8'), Some(CardValue::Num(8)));
        assert_eq!(CardValue::from_char('2'), Some(CardValue::Num(2)));
        assert_eq!(CardValue::from_char('3'), Some(CardValue::Num(3)));
    }

    #[test]
    fn test_card_value_sort() {
        let cards_input = "2A473KQ5J69T8";

        let mut cards: Vec<CardValue> = cards_input
            .chars()
            .map(|c| CardValue::from_char(c).unwrap())
            .collect();

        cards.sort();

        assert_eq!(cards.first(), Some(&CardValue::Num(2)));
        assert_eq!(cards.get(1), Some(&CardValue::Num(3)));
        assert_eq!(cards.get(2), Some(&CardValue::Num(4)));
        assert_eq!(cards.get(3), Some(&CardValue::Num(5)));
        assert_eq!(cards.get(4), Some(&CardValue::Num(6)));
        assert_eq!(cards.get(5), Some(&CardValue::Num(7)));
        assert_eq!(cards.get(6), Some(&CardValue::Num(8)));
        assert_eq!(cards.get(7), Some(&CardValue::Num(9)));
        assert_eq!(cards.get(8), Some(&CardValue::T));
        assert_eq!(cards.get(9), Some(&CardValue::J));
        assert_eq!(cards.get(10), Some(&CardValue::Q));
        assert_eq!(cards.get(11), Some(&CardValue::K));
        assert_eq!(cards.get(12), Some(&CardValue::A));
    }

    #[test]
    fn test_hand() {
        let mut v: Vec<HandType> = vec![
            HandType::HighCard,
            HandType::OnePair,
            HandType::TwoPair,
            HandType::ThreeOfAKind,
            HandType::FullHouse,
            HandType::FourOfAKind,
            HandType::FiveOfAKind,
        ];
        v.sort();

        assert_eq!(v.first(), Some(&HandType::HighCard));
        assert_eq!(v.get(1), Some(&HandType::OnePair));
        assert_eq!(v.get(2), Some(&HandType::TwoPair));
        assert_eq!(v.get(3), Some(&HandType::ThreeOfAKind));
        assert_eq!(v.get(4), Some(&HandType::FullHouse));
        assert_eq!(v.get(5), Some(&HandType::FourOfAKind));
        assert_eq!(v.get(6), Some(&HandType::FiveOfAKind));
    }
}
