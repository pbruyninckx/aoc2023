use anyhow::Error;
use std::fs;
use std::path::Path;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Tile {
    Rounded = 0,
    Cube = 1,
    Empty = 2,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            'O' => Tile::Rounded,
            '#' => Tile::Cube,
            '.' => Tile::Empty,
            _ => panic!("Not supposed to happen"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    let lines: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.chars().map(Tile::from_char).collect())
        .collect();

    let mut rows = vec![vec![Tile::Empty; lines.len()]; lines[0].len()];
    for (i, line) in lines.iter().enumerate() {
        for (j, tile) in line.iter().enumerate() {
            rows[j][i] = *tile;
        }
    }
    rows
}

fn sub_load(offset: usize, num_rounded: usize, total: usize) -> usize {
    if num_rounded > 0 {
        let start_val = total - offset;
        (start_val + start_val - num_rounded + 1) * num_rounded / 2
    } else {
        0
    }
}

fn solve_col(col: &[Tile]) -> usize {
    let mut num_rounded = 0;
    let mut offset = 0;
    let mut ret = 0;

    for (index, tile) in col.iter().enumerate() {
        match tile {
            Tile::Rounded => num_rounded += 1,
            Tile::Cube => {
                if num_rounded != 0 {
                    ret += sub_load(offset, num_rounded, col.len());
                    num_rounded = 0;
                }
                offset = index + 1;
            }
            Tile::Empty => {}
        }
    }
    if num_rounded != 0 {
        ret += sub_load(offset, num_rounded, col.len());
    }

    ret
}
fn solve(cols: &[Vec<Tile>]) -> usize {
    cols.iter().map(|c| solve_col(c)).sum()
}

fn main() -> Result<(), Error> {
    let cols = parse_input(&fs::read_to_string(Path::new("data/input14.txt"))?);
    println!("{}", solve(&cols));

    Ok(())
}
