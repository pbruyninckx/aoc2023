use crate::Data::{Number, Symbol};
use anyhow::Error;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const MAX_NUM_LENGTH: i32 = 3;

#[derive(PartialEq)]
enum Data {
    Symbol(char),
    Number(i64),
}

type Schematic = HashMap<(i32, i32), Data>;

fn num_length(mut num: i64) -> i32 {
    let mut ret = 0;
    while num > 0 {
        num /= 10;
        ret += 1
    }
    ret
}

fn extract_data(lines: &[&str]) -> Result<Schematic, Error> {
    let mut ret = HashMap::new();
    for (row, line) in lines.iter().enumerate() {
        let mut number: Option<i64> = None;
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                number = Some(
                    number.unwrap_or(0) * 10
                        + c.to_digit(10).ok_or(Error::msg("Not a digit"))? as i64,
                );
            } else {
                if let Some(finished_number) = number {
                    ret.insert(
                        (row as i32, col as i32 - num_length(finished_number)),
                        Number(finished_number),
                    );
                    number = None;
                }
                if c != '.' {
                    ret.insert((row as i32, col as i32), Symbol(c));
                }
            }
        }
        if let Some(finished_number) = number {
            ret.insert(
                (row as i32, line.len() as i32 - num_length(finished_number)),
                Number(finished_number),
            );
        }
    }
    Ok(ret)
}

fn contains_symbol(data: &Schematic, pos: (i32, i32)) -> bool {
    matches!(data.get(&pos), Some(Symbol(_)))
}

fn touching_symbol(data: &Schematic, pos: (i32, i32), number: i64) -> bool {
    let right_pos = (pos.0, pos.1 + num_length(number) - 1);
    for dr in -1..2 {
        for dc in -1..2 {
            if contains_symbol(data, (pos.0 + dr, pos.1 + dc))
                || contains_symbol(data, (right_pos.0 + dr, right_pos.1 + dc))
            {
                return true;
            }
        }
    }
    false
}

fn get_gear_ratio(data: &Schematic, pos: (i32, i32)) -> Option<i64> {
    if data.get(&pos) != Some(&Symbol('*')) {
        return None;
    }

    let mut numbers = vec![];
    for row in pos.0 - 1..pos.0 + 2 {
        for col in pos.1 - MAX_NUM_LENGTH..pos.1 + 2 {
            if let Some(Number(number)) = data.get(&(row, col)) {
                if col + num_length(*number) >= pos.1 {
                    numbers.push(*number);
                }
            }
        }
    }

    if numbers.len() == 2 {
        Some(numbers[0] * numbers[1])
    } else {
        None
    }
}

fn solve(data: &Schematic) -> i64 {
    data.iter()
        .filter_map(|data_point| {
            if let Number(number) = *data_point.1 {
                if touching_symbol(data, *data_point.0, number) {
                    Some(number)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum()
}

fn solve2(data: &Schematic) -> i64 {
    data.iter()
        .filter_map(|data_point| get_gear_ratio(data, *data_point.0))
        .sum()
}

fn main() -> Result<(), Error> {
    let file_name = Path::new("data/input03.txt");
    let file_data = fs::read_to_string(file_name)?;
    let lines: Vec<_> = file_data.lines().collect();
    let data = extract_data(&lines)?;
    println!("{}", solve(&data));
    println!("{}", solve2(&data));

    Ok(())
}
