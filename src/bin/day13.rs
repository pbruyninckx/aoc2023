use anyhow::Error;
use std::fs;
use std::iter::zip;
use std::path::Path;

struct Pattern {
    rows: Vec<String>,
    cols: Vec<String>,
}

impl Pattern {
    fn from_str(input: &str) -> Self {
        let rows: Vec<String> = input.lines().map(|s| String::from(s.trim())).collect();
        let mut cols = vec![String::new(); rows[0].len()];
        for row in rows.iter() {
            for (j, c) in row.chars().enumerate() {
                cols[j].push(c)
            }
        }

        Self { rows, cols }
    }
}

fn parse_input(input: &str) -> Vec<Pattern> {
    input.split("\n\n").map(Pattern::from_str).collect()
}

fn mirror_index(lines: &Vec<String>) -> Option<usize> {
    for i in 0..lines.len() - 1 {
        if lines[i] == lines[i + 1]
            && zip(lines[0..i].iter().rev(), lines[i + 2..].iter()).all(|(lhs, rhs)| lhs == rhs)
        {
            return Some(i + 1);
        }
    }
    None
}

fn solve(input: &[Pattern]) -> usize {
    input
        .iter()
        .map(|p| mirror_index(&p.cols).unwrap_or_else(|| mirror_index(&p.rows).unwrap() * 100))
        .sum()
}

fn main() -> Result<(), Error> {
    let input = parse_input(&fs::read_to_string(Path::new("data/input13.txt"))?);
    println!("{}", solve(&input));
    Ok(())
}
