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

type SequenceSolveFn = fn(&[Vec<i64>]) -> i64;

fn solve_line(numbers: &Vec<i64>, sequence_solve_fn: SequenceSolveFn) -> i64 {
    let mut sequences = vec![numbers.to_owned()];
    while !sequences.last().unwrap().iter().all(|num| *num == 0) {
        sequences.push(compute_diffs(sequences.last().unwrap()));
    }
    sequence_solve_fn(&sequences)
}

fn next_end_value(sequences: &[Vec<i64>]) -> i64 {
    sequences.iter().map(|s| *s.last().unwrap()).sum()
}

fn next_start_value(sequences: &[Vec<i64>]) -> i64 {
    sequences
        .iter()
        .map(|s| *s.first().unwrap())
        .rev()
        .reduce(|acc, v| v - acc)
        .unwrap()
}

fn solve(input: &[Vec<i64>], sequence_solve_fn: SequenceSolveFn) -> i64 {
    input.iter().map(|l| solve_line(l, sequence_solve_fn)).sum()
}

fn main() -> Result<(), Error> {
    let input: Vec<_> = fs::read_to_string(Path::new("data/input09.txt"))?
        .lines()
        .map(parse_numbers)
        .collect::<Result<_, _>>()?;
    println!("{}", solve(&input, next_end_value));
    println!("{}", solve(&input, next_start_value));

    Ok(())
}
