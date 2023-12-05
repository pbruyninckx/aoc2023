use anyhow::Error;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

struct Card {
    winning: Vec<i64>,
    mine: Vec<i64>,
}

fn parse_numbers(string: &str) -> Result<Vec<i64>, Error> {
    Ok(string
        .trim()
        .split_ascii_whitespace()
        .map(|d| d.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?)
}

impl Card {
    fn from_str(string: &str) -> Result<Self, Error> {
        let parts = string.split([':', '|']).collect::<Vec<_>>();
        let winning = parse_numbers(parts.get(1).ok_or(Error::msg("winning numbers missing"))?)?;
        let mine = parse_numbers(parts.get(2).ok_or(Error::msg("my numbers missing"))?)?;
        Ok(Self { winning, mine })
    }

    fn get_points(&self) -> i64 {
        let winning: HashSet<i64> = HashSet::from_iter(self.winning.iter().cloned());
        let num_matches: u32 = self
            .mine
            .iter()
            .map(|number| if winning.contains(number) { 1 } else { 0 })
            .sum();

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
fn main() -> Result<(), Error> {
    let cards = fs::read_to_string(Path::new("data/input04.txt"))?
        .lines()
        .map(Card::from_str)
        .collect::<Result<Vec<Card>, _>>()?;
    println!("{}", solve(&cards));
    Ok(())
}
