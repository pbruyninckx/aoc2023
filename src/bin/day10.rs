use anyhow::Error;
use std::fs;
use std::path::Path;

struct Input {
    map: Vec<Vec<char>>,
    start: (i64, i64),
}

fn inverse(direction: &(i64, i64)) -> (i64, i64) {
    (-direction.0, -direction.1)
}

const DIRECTIONS: [(i64, i64); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn get_directions(c: char) -> Option<[(i64, i64); 2]> {
    Some(match c {
        '|' => [(-1, 0), (1, 0)],
        '-' => [(0, -1), (0, 1)],
        'L' => [(-1, 0), (0, 1)],
        'J' => [(-1, 0), (0, -1)],
        '7' => [(0, -1), (1, 0)],
        'F' => [(0, 1), (1, 0)],
        _ => None?,
    })
}

fn parse_input(input: &str) -> Input {
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    // Find start
    let start: (i64, i64) = {
        let mut start: Option<_> = None;
        'row: for row in 0..map.len() as i64 {
            for col in 0..map[0].len() as i64 {
                if map[row as usize][col as usize] == 'S' {
                    start = Some((row, col));
                    break 'row;
                }
            }
        }
        start.unwrap()
    };
    // Replace start
    let mut needed_directions = vec![];
    for (dr, dc) in DIRECTIONS {
        if let Some(dirs) = get_directions(map[(start.0 + dr) as usize][(start.1 + dc) as usize]) {
            if dirs.contains(&(-dr, -dc)) {
                needed_directions.push((dr, dc))
            }
        }
    }
    assert_eq!(needed_directions.len(), 2);
    for tile in "-|7LFJ".chars() {
        let dirs = get_directions(tile).unwrap();
        if dirs.contains(&needed_directions[0]) && dirs.contains(&needed_directions[1]) {
            map[start.0 as usize][start.1 as usize] = tile;
        }
    }

    Input { map, start }
}

fn next_pos(
    map: &Vec<Vec<char>>,
    pos: &(i64, i64),
    last_dir: &(i64, i64),
) -> ((i64, i64), (i64, i64)) {
    let next_directions = get_directions(map[pos.0 as usize][pos.1 as usize]).unwrap();
    let next_dir = {
        if next_directions[0] == inverse(last_dir) {
            next_directions[1]
        } else {
            next_directions[0]
        }
    };
    ((pos.0 + next_dir.0, pos.1 + next_dir.1), next_dir)
}

fn solve(input: &Input) -> i64 {
    let start_dirs =
        get_directions(input.map[input.start.0 as usize][input.start.1 as usize]).unwrap();
    let mut last_forward_dir = start_dirs[0];
    let mut last_backward_dir = start_dirs[1];
    let mut forward_pos = (
        input.start.0 + start_dirs[0].0,
        input.start.1 + start_dirs[0].1,
    );
    let mut backward_pos = (
        input.start.0 + start_dirs[1].0,
        input.start.1 + start_dirs[1].1,
    );
    let mut counter = 1;

    while forward_pos != backward_pos {
        (forward_pos, last_forward_dir) = next_pos(&input.map, &forward_pos, &last_forward_dir);
        (backward_pos, last_backward_dir) = next_pos(&input.map, &backward_pos, &last_backward_dir);
        counter += 1;
    }

    counter
}

fn main() -> Result<(), Error> {
    let input = parse_input(&fs::read_to_string(Path::new("data/input10.txt"))?);
    println!("{}", solve(&input));

    Ok(())
}
