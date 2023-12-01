use anyhow::Error;
use std::fs::read_to_string;
use std::path::Path;

const DIGITS: [(&str, u32); 18] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];
fn get_spelled_calibration_value(line: &String) -> u32 {
    let mut first_digit: u32 = 0;
    'outer: for i in 0..line.len() {
        for (d, v) in DIGITS.iter() {
            if line[i..].starts_with(*d) {
                first_digit = *v;
                break 'outer;
            }
        }
    }
    let mut last_digit: u32 = 0;
    'outer: for i in 0..line.len() {
        for (d, v) in DIGITS.iter() {
            if line[line.len() - i - 1..].starts_with(*d) {
                last_digit = *v;
                break 'outer;
            }
        }
    }

    first_digit * 10 + last_digit
}

#[allow(clippy::ptr_arg)]
fn get_calibration_value(line: &String) -> u32 {
    let digits: Vec<_> = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    10 * digits.first().unwrap() + digits.last().unwrap()
}

fn solve(input: &[String], compute_calibration: fn(&String) -> u32) -> u32 {
    input.iter().map(compute_calibration).sum()
}

fn main() -> Result<(), Error> {
    let input: Vec<_> = read_to_string(Path::new("data/input01.txt"))?
        .lines()
        .map(String::from)
        .collect();
    println!("{}", solve(&input, get_calibration_value));
    println!("{}", solve(&input, get_spelled_calibration_value));
    Ok(())
}
