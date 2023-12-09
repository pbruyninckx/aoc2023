use anyhow::Error;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

enum Direction {
    Left = 0,
    Right = 1,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Not a LR direction"),
        }
    }
}

struct Map {
    instructions: Vec<Direction>,
    network: HashMap<String, (String, String)>,
}

fn parse_node(line: &str) -> (String, (String, String)) {
    let parts: Vec<_> = line.split(|c: char| !c.is_ascii_alphabetic()).collect();
    (
        String::from(parts[0]),
        (String::from(parts[4]), String::from(parts[6])),
    )
}

fn parse_input(input: &str) -> Map {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(Direction::from_char)
        .collect();

    let network = lines.skip(1).map(parse_node).collect();

    Map {
        instructions,
        network,
    }
}

fn solve(map: &Map, start_node: &str, is_end_node: fn(&str) -> bool) -> u64 {
    let cycle_instructions = map.instructions.iter().cycle();

    let mut current_node = String::from(start_node);
    let mut num_steps = 0;
    for instruction in cycle_instructions {
        let current_paths = map.network.get(&*current_node).unwrap();
        current_node = match instruction {
            Direction::Left => current_paths.0.clone(),
            Direction::Right => current_paths.1.clone(),
        };
        num_steps += 1;

        if is_end_node(&current_node) {
            break;
        }
    }
    num_steps
}

fn main() -> Result<(), Error> {
    let map = parse_input(&fs::read_to_string(Path::new("data/input08.txt"))?);
    println!("{}", solve(&map, "AAA", |s| s == "ZZZ"));

    Ok(())
}
