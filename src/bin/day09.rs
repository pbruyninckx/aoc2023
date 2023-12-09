use anyhow::Error;
use aoc2023::parse_numbers;
use std::fs;
use std::iter::zip;
use std::path::Path;

fn compute_diffs(numbers: &[i64]) -> Vec<i64> {
    zip(numbers.iter().skip(1), numbers.iter())
        .map(|(a, b)| *a - *b)
        .collect()
}

fn solve_line(numbers: &Vec<i64>) -> i64 {
    let mut sequences = vec![numbers.to_owned()];
    while !sequences.last().unwrap().iter().all(|num| *num == 0) {
        sequences.push(compute_diffs(sequences.last().unwrap()));
    }
    sequences.iter().map(|s| *s.last().unwrap()).sum()
}

fn solve(input: &[Vec<i64>]) -> i64 {
    input.iter().map(solve_line).sum()
}

fn main() -> Result<(), Error> {
    let input: Vec<_> = fs::read_to_string(Path::new("data/input09.txt"))?
        .lines()
        .map(parse_numbers)
        .collect::<Result<_, _>>()?;
    println!("{}", solve(&input));

    Ok(())
}
