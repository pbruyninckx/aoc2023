use anyhow::Error;
use aoc2023::parse_numbers;
use std::fs;
use std::iter::zip;
use std::path::Path;

struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn num_ways_to_beat(&self) -> u64 {
        (0..self.time + 1)
            .map(|p| p * (self.time - p))
            .filter(|d| *d > self.distance)
            .count() as u64
    }
}

fn parse_input(input: &str) -> Result<Vec<Race>, Error> {
    let lines = input.lines().collect::<Vec<_>>();
    let times = parse_numbers(
        lines
            .first()
            .ok_or(Error::msg("no times line"))?
            .strip_prefix("Time:")
            .ok_or(Error::msg("No time prefix"))?,
    )?;
    let distances = parse_numbers(
        lines
            .get(1)
            .ok_or(Error::msg("no distance line"))?
            .strip_prefix("Distance:")
            .ok_or(Error::msg("No distance prefix"))?,
    )?;
    Ok(zip(times, distances)
        .map(|(time, distance)| Race { time, distance })
        .collect())
}

fn solve(races: &[Race]) -> u64 {
    races.iter().map(Race::num_ways_to_beat).product()
}

fn main() -> Result<(), Error> {
    let races = parse_input(&fs::read_to_string(Path::new("data/input06.txt"))?)?;
    println!("{}", solve(&races));
    Ok(())
}
