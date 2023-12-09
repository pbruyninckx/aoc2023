use anyhow::Error;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash, Debug)]
struct Card(i8);

impl Card {
    fn from_char(c: char) -> Self {
        Self(c.to_digit(10).unwrap_or_else(|| match c {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("invalid card"),
        }) as i8)
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    High = 0,
    OnePair = 1,
    TwoPair = 2,
    Three = 3,
    FullHouse = 4,
    Four = 5,
    Five = 6,
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let sorted_counts = {
            let mut histogram: HashMap<Card, i8> = HashMap::new();
            for card in self.cards {
                histogram.entry(card).and_modify(|e| *e += 1).or_insert(1);
            }

            let mut counts: Vec<i8> = histogram.values().cloned().collect();
            counts.sort_by_key(|c| -*c);
            counts
        };

        match sorted_counts.len() {
            1 => HandType::Five,
            2 => {
                if sorted_counts[0] == 4 {
                    HandType::Four
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if sorted_counts[0] == 3 {
                    HandType::Three
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::High,
            _ => panic!("Logic error"),
        }
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.hand_type(), self.cards).cmp(&(other.hand_type(), other.cards))
    }
}

#[derive(Clone, Debug)]
struct HandWithBid {
    hand: Hand,
    bid: u64,
}

fn parse_line(line: &str) -> Result<HandWithBid, Error> {
    let (hand_str, bid_str) = line
        .split_once(' ')
        .ok_or(Error::msg("Two parts expected"))?;
    let card_vec: Vec<_> = hand_str.chars().map(Card::from_char).collect();
    let cards = card_vec
        .try_into()
        .map_err(|_| Error::msg("Five cards expected"))?;
    let hand = Hand { cards };
    let bid = bid_str.parse::<u64>()?;

    Ok(HandWithBid { hand, bid })
}

fn parse_input(input: &str) -> Result<Vec<HandWithBid>, Error> {
    input.lines().map(parse_line).collect()
}

fn solve(input: Vec<HandWithBid>, cmp_hands: fn(&Hand, &Hand)->Ordering) -> u64 {
    let mut sorted_hands = input.to_vec();
    sorted_hands.sort_by(|a, b| cmp_hands(&a.hand, &b.hand));

    sorted_hands
        .iter()
        .enumerate()
        .map(|(i, hb)| (i + 1) as u64 * hb.bid)
        .sum()
}
fn main() -> Result<(), Error> {
    let input = parse_input(&fs::read_to_string(Path::new("data/input07.txt"))?)?;

    println!("{}", solve(input, Hand::cmp));
    Ok(())
}
