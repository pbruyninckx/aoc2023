use anyhow::Error;
use std::fs::read_to_string;
use std::path::Path;

fn get_calibration_value(line: &String) -> u32 {
    let digits: Vec<_> = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    10 * digits.first().unwrap() + digits.last().unwrap()
}

fn solve1(input: &[String]) -> u32 {
    input.iter().map(get_calibration_value).sum()
}

fn main() -> Result<(), Error> {
    let input: Vec<_> = read_to_string(Path::new("data/input01.txt"))?
        .lines()
        .map(String::from)
        .collect();
    println!("{}", solve1(&input));
    Ok(())
}
