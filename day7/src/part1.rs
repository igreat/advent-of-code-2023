use std::cmp::Ordering;

fn parse_to_u16(s: &str) -> u16 {
    s.as_bytes()
        .iter()
        .fold(0u16, |acc, &b| acc * 10 + (b - b'0') as u16)
}

pub fn part1() {
    let input = std::fs::read_to_string("day7_input.txt").unwrap();
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|l| Hand::new(&l[0..5], parse_to_u16(&l[6..])))
        .collect();

    hands.sort_by(|a, b| a.cmp(b));

    let sum = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u32 * h.bid as u32)
        .sum::<u32>();

    println!("Part 1: {}", sum);
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
        b'2'..=b'9' => letter - b'2',
        b'A' => 12,
        b'K' => 11,
        b'Q' => 10,
        b'J' => 9,
        b'T' => 8,
        _ => 0,
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct Hand {
    id: String,
    hand_type: HandType,
    bid: u16,
}

impl Hand {
    fn new(id: &str, bid: u16) -> Hand {
        let mut cards = [0 as u8; 13];
        let mut num_unique_cards = 0;
        for card in id.as_bytes() {
            if cards[strength(*card) as usize] == 0 {
                num_unique_cards += 1;
            }
            cards[strength(*card) as usize] += 1;
        }

        let hand_type = match num_unique_cards {
            1 => HandType::FiveOfAKind,
            2 => {
                if cards.contains(&4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if cards.contains(&3) {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compare_hands1() {
        let hand1 = Hand::new("QQQJA", 0);
        let hand2 = Hand::new("T55J5", 0);
        assert_eq!(hand1.cmp(&hand2), Ordering::Greater);
    }

    #[test]
    fn compare_hands2() {
        let hand1 = Hand::new("KK677", 0);
        let hand2 = Hand::new("32T3K", 0);
        assert_eq!(hand1.cmp(&hand2), Ordering::Greater);
    }

    #[test]
    fn compare_hands3() {
        let hand1 = Hand::new("KK677", 0);
        let hand2 = Hand::new("KTJJT", 0);
        assert_eq!(hand1.cmp(&hand2), Ordering::Greater);
    }

    #[test]
    fn compare_hands4() {
        let hand1 = Hand::new("T55J5", 0);
        let hand2 = Hand::new("KK677", 0);
        assert_eq!(hand1.cmp(&hand2), Ordering::Greater);
    }
}
