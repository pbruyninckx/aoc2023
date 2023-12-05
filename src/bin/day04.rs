use anyhow::Error;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

struct Card {
    winning: Vec<i64>,
    mine: Vec<i64>,
}

impl Card {
    fn from_str(string: &str) -> Result<Self, Error> {
        let parts = string.split([':', '|']).collect::<Vec<_>>();
        let winning =
            aoc2023::parse_numbers(parts.get(1).ok_or(Error::msg("winning numbers missing"))?)?;
        let mine = aoc2023::parse_numbers(parts.get(2).ok_or(Error::msg("my numbers missing"))?)?;
        Ok(Self { winning, mine })
    }

    fn get_num_matches(&self) -> u32 {
        let winning: HashSet<i64> = HashSet::from_iter(self.winning.iter().cloned());
        self.mine
            .iter()
            .map(|number| if winning.contains(number) { 1 } else { 0 })
            .sum()
    }

    fn get_points(&self) -> i64 {
        let num_matches = self.get_num_matches();

        if num_matches == 0 {
            0
        } else {
            2_i64.pow(num_matches - 1)
        }
    }
}

fn solve(cards: &[Card]) -> i64 {
    cards.iter().map(|c| c.get_points()).sum()
}

fn solve2(cards: &[Card]) -> usize {
    let mut num_cards = vec![1_usize; cards.len()];

    for (card_index, card) in cards.iter().enumerate().rev().skip(1) {
        let num_matches = card.get_num_matches() as usize;
        num_cards[card_index] += num_cards[card_index + 1..card_index + 1 + num_matches]
            .iter()
            .sum::<usize>();
    }

    num_cards.iter().sum()
}

fn main() -> Result<(), Error> {
    let cards = fs::read_to_string(Path::new("data/input04.txt"))?
        .lines()
        .map(Card::from_str)
        .collect::<Result<Vec<Card>, _>>()?;
    println!("{}", solve(&cards));
    println!("{}", solve2(&cards));
    Ok(())
}
