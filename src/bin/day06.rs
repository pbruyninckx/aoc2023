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

fn parse_as_big_number(line: &str) -> i64 {
    line.chars().fold(0_i64, |acc, c| {
        c.to_digit(10).map_or(acc, |d| acc * 10 + d as i64)
    })
}
fn parse_input_as_single_race(input: &str) -> Result<Race, Error> {
    let values: Vec<_> = input.lines().map(parse_as_big_number).collect();
    Ok(Race {
        time: *values.first().ok_or(Error::msg("no time line"))?,
        distance: *values.get(1).ok_or(Error::msg("no distance line"))?,
    })
}

fn solve(races: &[Race]) -> u64 {
    races.iter().map(Race::num_ways_to_beat).product()
}

fn main() -> Result<(), Error> {
    let input = fs::read_to_string(Path::new("data/input06.txt"))?;
    let races = parse_input(&input)?;
    println!("{}", solve(&races));
    let mega_race = parse_input_as_single_race(&input)?;
    println!("{}", mega_race.num_ways_to_beat());
    Ok(())
}
