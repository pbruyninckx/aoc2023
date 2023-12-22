use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct LensInfo {
    position: usize,
    focal_length: usize,
}

fn hash(step: &str) -> usize {
    step.chars()
        .fold(0, |ret, c| ((ret + (c as usize)) * 17) % 256)
}

#[derive(Copy, Clone)]
enum Action {
    Remove,
    Update(usize),
}

struct Instruction {
    label: String,
    action: Action,
}

impl Instruction {
    fn from_str(string: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-z]+)(-|=)(\d*)").unwrap();
        }
        let groups = RE.captures(string).unwrap();
        Self {
            label: String::from(groups.get(1).unwrap().as_str()),
            action: if groups.get(2).unwrap().as_str() == "-" {
                Action::Remove
            } else {
                Action::Update(groups.get(3).unwrap().as_str().parse().unwrap())
            },
        }
    }
}

fn solve(sequence: &[&str]) -> usize {
    sequence.iter().map(|&s| hash(s)).sum()
}

fn solve2(sequence: &[&str]) -> usize {
    let instructions = sequence.iter().map(|&s| Instruction::from_str(s));
    let mut lens_by_label: HashMap<String, LensInfo> = HashMap::new();

    for (index, instruction) in instructions.enumerate() {
        match instruction.action {
            Action::Remove => {
                lens_by_label.remove(&instruction.label);
            }
            Action::Update(fl) => {
                lens_by_label
                    .entry(instruction.label)
                    .and_modify(|e| {
                        e.focal_length = fl;
                    })
                    .or_insert(LensInfo {
                        position: index,
                        focal_length: fl,
                    });
            }
        }
    }
    let mut lenses: Vec<_> = lens_by_label.iter().collect();
    lenses.sort_by_key(|(_, info)| info.position);

    let mut boxes: Vec<Vec<usize>> = vec![vec![]; 256];

    for (label, info) in lenses.iter() {
        boxes[hash(label)].push(info.focal_length);
    }

    boxes
        .iter()
        .enumerate()
        .map(|(box_ind, fls)| -> usize {
            fls.iter()
                .enumerate()
                .map(|(slot, focal_length)| (1 + box_ind) * (1 + slot) * focal_length)
                .sum()
        })
        .sum()
}

fn main() {
    let raw_input = fs::read_to_string(Path::new("data/input15.txt")).unwrap();
    let input: Vec<_> = raw_input.trim().split(',').collect();

    println!("{}", solve(&input));
    println!("{}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }
}
