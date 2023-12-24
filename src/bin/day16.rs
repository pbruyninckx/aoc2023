use std::collections::HashSet;
use std::fs;
use std::ops::{Add, AddAssign, Index};
use std::path::Path;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    r: i64,
    c: i64,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Direction {
    dr: i64,
    dc: i64,
}

impl Add<Direction> for Pos {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        Self {
            r: self.r + rhs.dr,
            c: self.c + rhs.dc,
        }
    }
}

impl AddAssign<Direction> for Pos {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

struct Map {
    data: Vec<char>,
    nr: i64,
    nc: i64,
}

impl Map {
    fn from_str(string: &str) -> Self {
        let data: Vec<_> = string.chars().filter(|c| !c.is_whitespace()).collect();
        let nr = string.lines().count() as i64;
        let nc = data.len() as i64 / nr;
        assert_eq!(nr * nc, data.len() as i64);
        Self { data, nr, nc }
    }

    fn contains(&self, pos: &Pos) -> bool {
        pos.r >= 0 && pos.r < self.nr && pos.c >= 0 && pos.c < self.nc
    }
}

impl Index<Pos> for Map {
    type Output = char;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.data[(self.nr * pos.r + pos.c) as usize]
    }
}

fn new_directions(dir: &Direction, m: char) -> Vec<Direction> {
    match m {
        '/' => vec![Direction {
            dr: -dir.dc,
            dc: -dir.dr,
        }],
        '\\' => vec![Direction {
            dr: dir.dc,
            dc: dir.dr,
        }],
        '-' => {
            if dir.dr == 0 {
                vec![*dir]
            } else {
                vec![Direction { dr: 0, dc: 1 }, Direction { dr: 0, dc: -1 }]
            }
        }
        '|' => {
            if dir.dc == 0 {
                vec![*dir]
            } else {
                vec![Direction { dr: -1, dc: 0 }, Direction { dr: 1, dc: 0 }]
            }
        }
        _ => panic!("Unexpected input character"),
    }
}

fn solve(map: &Map) -> usize {
    let mut active = vec![(Pos { r: 0, c: -1 }, Direction { dr: 0, dc: 1 })];
    let mut seen: HashSet<(Pos, Direction)> = HashSet::new();

    while let Some((mut current_pos, current_dir)) = active.pop() {
        loop {
            current_pos += current_dir;

            if !map.contains(&current_pos) || seen.contains(&(current_pos, current_dir)) {
                break;
            }

            seen.insert((current_pos, current_dir));

            if map[current_pos] != '.' {
                for new_dir in new_directions(&current_dir, map[current_pos]) {
                    active.push((current_pos, new_dir));
                }
                break;
            }
        }
    }

    let seen_pos: HashSet<&Pos> = seen.iter().map(|(pos, _)| pos).collect();
    seen_pos.len()
}

fn main() {
    let map = Map::from_str(&fs::read_to_string(Path::new("data/input16.txt")).unwrap());

    println!("{}", solve(&map));
}
