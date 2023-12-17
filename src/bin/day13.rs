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

fn count_diffs(a: &str, b: &str) -> usize {
    zip(a.chars(), b.chars())
        .map(|(ac, bc)| if ac == bc { 0 } else { 1 })
        .sum()
}

fn mirror_index(lines: &Vec<String>, num_diffs: usize) -> Option<usize> {
    for i in 0..lines.len() - 1 {
        if count_diffs(&lines[i], &lines[i + 1]) <= num_diffs
            && zip(lines[0..i + 1].iter().rev(), lines[i + 1..].iter())
                .map(|(lhs, rhs)| count_diffs(lhs, rhs))
                .sum::<usize>()
                == num_diffs
        {
            return Some(i + 1);
        }
    }
    None
}

fn solve(input: &[Pattern], num_diffs: usize) -> usize {
    input
        .iter()
        .map(|p| {
            mirror_index(&p.cols, num_diffs)
                .unwrap_or_else(|| mirror_index(&p.rows, num_diffs).unwrap() * 100)
        })
        .sum()
}

fn main() -> Result<(), Error> {
    let input = parse_input(&fs::read_to_string(Path::new("data/input13.txt"))?);
    println!("{}", solve(&input, 0));
    println!("{}", solve(&input, 1));
    Ok(())
}
