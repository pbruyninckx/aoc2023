use anyhow::Error;
use std::fs;
use std::path::Path;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
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

fn score_col(col: &[Tile]) -> usize {
    col.iter()
        .enumerate()
        .map(|(i, &tile)| {
            if tile == Tile::Rounded {
                col.len() - i
            } else {
                0
            }
        })
        .sum()
}

fn score(map: &[Vec<Tile>]) -> usize {
    map.iter().map(|c| score_col(c)).sum()
}
fn solve(cols: &[Vec<Tile>]) -> usize {
    cols.iter().map(|c| solve_col(c)).sum()
}

fn roll_col(col: &mut [Tile]) {
    let mut num_rounded = 0;
    let mut offset = 0;

    for index in 0..col.len() {
        let tile = col[index];
        match tile {
            Tile::Rounded => {
                num_rounded += 1;
                col[index] = Tile::Empty;
            }
            Tile::Cube => {
                if num_rounded != 0 {
                    col[offset..offset + num_rounded].fill(Tile::Rounded);
                    num_rounded = 0;
                }
                offset = index + 1;
            }
            Tile::Empty => {}
        }
    }
    if num_rounded != 0 {
        col[offset..offset + num_rounded].fill(Tile::Rounded);
    }
}

fn roll(map: &mut [Vec<Tile>]) {
    for col in map.iter_mut() {
        roll_col(col);
    }
}
fn rotate(map: &mut [Vec<Tile>]) {
    let n = map.len();
    for r in 0..n / 2 {
        for c in 0..n / 2 {
            let backup_tile = map[r][c];
            map[r][c] = map[c][n - 1 - r];
            map[c][n - 1 - r] = map[n - 1 - r][n - 1 - c];
            map[n - 1 - r][n - 1 - c] = map[n - 1 - c][r];
            map[n - 1 - c][r] = backup_tile;
        }
    }
}

fn cycle(map: &mut [Vec<Tile>]) {
    for _ in 0..4 {
        roll(map);
        rotate(map);
    }
}

fn find_period(vals: &[usize]) -> usize {
    let mut ret = 2;
    for ind in vals.len() / 4..vals.len() / 2 {
        while vals[ind + ret] != vals[ind] {
            ret += 1;
        }
    }
    ret
}

fn solve2(map: &mut [Vec<Tile>]) -> usize {
    let mut loads = vec![0];
    for _ in 0..400 {
        cycle(map);
        loads.push(score(map));
    }

    let period = find_period(&loads);
    loads[(1000000000 % period) + (200 / period) * period]
}

fn main() -> Result<(), Error> {
    let mut cols = parse_input(&fs::read_to_string(Path::new("data/input14.txt"))?);
    roll(&mut cols);
    println!("{}", solve(&cols));
    println!("{}", solve2(&mut cols));
    Ok(())
}
