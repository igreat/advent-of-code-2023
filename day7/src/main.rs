use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    part1();
    // part2();
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn strength(letter: u8) -> u8 {
    match letter {
        b'2'..=b'9' => letter - b'0',
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' => 11,
        b'T' => 10,
        _ => panic!("Invalid letter"),
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct Hand {
    id: String,
    hand_type: HandType,
    bid: u32,
}

impl Hand {
    fn new(id: &str, bid: u32) -> Hand {
        let mut cards = HashMap::new();
        for card in id.as_bytes() {
            *cards.entry(card).or_insert(0) += 1;
        }

        let hand_type = match cards.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if cards.values().any(|&v| v == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if cards.values().any(|&v| v == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        };

        Hand {
            id: id.to_string(),
            hand_type,
            bid,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for (&s, &o) in self.id.as_bytes().iter().zip(other.id.as_bytes().iter()) {
                    if s != o {
                        if strength(s) > strength(o) {
                            return Ordering::Greater;
                        } else {
                            return Ordering::Less;
                        }
                    }
                }
                Ordering::Equal
            }
        }
    }
}

fn part1() {
    let input = std::fs::read_to_string("day7_input.txt").unwrap();
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|l| {
            let mut hand_bid = l.split_whitespace();
            Hand::new(
                hand_bid.next().unwrap(),
                hand_bid.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();

    // sort by the comparison function in Hand
    hands.sort_by(|a, b| a.cmp(b));

    // dot product of bids and ranks
    let sum = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u32 * h.bid)
        .sum::<u32>();

    println!("Part 1: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_greater_than1() {
        let hand1 = Hand::new("QQQJA", 0);
        let hand2 = Hand::new("T55J5", 0);
        assert_eq!(hand1.cmp(&hand2), Ordering::Greater);
    }

    #[test]
    fn test_greater_than2() {
        let hand1 = Hand::new("KK677", 0);
        let hand2 = Hand::new("32T3K", 0);
        assert_eq!(hand1.cmp(&hand2), Ordering::Greater);
    }

    #[test]
    fn test_greater_than3() {
        let hand1 = Hand::new("KK677", 0);
        let hand2 = Hand::new("KTJJT", 0);
        assert_eq!(hand1.cmp(&hand2), Ordering::Greater);
    }

    #[test]
    fn test_greater_than4() {
        let hand1 = Hand::new("T55J5", 0);
        let hand2 = Hand::new("KK677", 0);
        assert_eq!(hand1.cmp(&hand2), Ordering::Greater);
    }
}
