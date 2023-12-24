use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;
use std::ops::{Add, Index, IndexMut, Sub};
use std::path::Path;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
struct Pos {
    r: i64,
    c: i64,
}

struct Map<T> {
    data: Vec<T>,
    nr: i64,
    nc: i64,
}

impl Map<i64> {
    fn from_str(string: &str) -> Self {
        let data: Vec<_> = string
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as i64))
            .collect();
        let nr = string.lines().count() as i64;
        let nc = data.len() as i64 / nr;
        assert_eq!(nr * nc, data.len() as i64);
        Self { data, nr, nc }
    }
}

impl<T> Map<T> {
    fn contains(&self, pos: &Pos) -> bool {
        pos.r >= 0 && pos.r < self.nr && pos.c >= 0 && pos.c < self.nc
    }
}

impl<T> Index<Pos> for Map<T> {
    type Output = T;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.data[(self.nr * pos.r + pos.c) as usize]
    }
}

impl<T> IndexMut<Pos> for Map<T> {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self.data[(self.nr * pos.r + pos.c) as usize]
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, EnumIter, Hash, Debug)]
enum Direction {
    Horizontal = 0,
    Vertical = 1,
}

impl Add<Direction> for Pos {
    type Output = Pos;

    fn add(self, rhs: Direction) -> Self::Output {
        Pos {
            r: self.r + if rhs == Direction::Vertical { 1 } else { 0 },
            c: self.c + if rhs == Direction::Horizontal { 1 } else { 0 },
        }
    }
}

impl Sub<Direction> for Pos {
    type Output = Pos;

    fn sub(self, rhs: Direction) -> Self::Output {
        Pos {
            r: self.r - if rhs == Direction::Vertical { 1 } else { 0 },
            c: self.c - if rhs == Direction::Horizontal { 1 } else { 0 },
        }
    }
}

impl Direction {
    fn switch(self) -> Self {
        match self {
            Direction::Horizontal => Direction::Vertical,
            Direction::Vertical => Direction::Horizontal,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct HeatState {
    heat_loss: i64,
    pos: Pos,
    last_dir: Direction,
}

impl Ord for HeatState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Negative ordering, as the heap is a max heap
        // Other items are included to have a total ordering
        (-self.heat_loss, self.last_dir, self.pos).cmp(&(
            -other.heat_loss,
            other.last_dir,
            self.pos,
        ))
    }
}

impl PartialOrd for HeatState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(map: &Map<i64>, skip: usize, take: usize) -> i64 {
    let target_pos = Pos {
        r: map.nr - 1,
        c: map.nc - 1,
    };
    let mut finished = HashSet::<(Pos, Direction)>::new();
    let mut todo = BinaryHeap::from_iter(Direction::iter().map(|last_dir| HeatState {
        heat_loss: 0,
        last_dir,
        pos: Pos { r: 0, c: 0 },
    }));

    while todo.peek().unwrap().pos != target_pos {
        let current = todo.pop().unwrap();
        if finished.contains(&(current.pos, current.last_dir)) {
            continue;
        }
        finished.insert((current.pos, current.last_dir));

        let new_dir = current.last_dir.switch();
        let mut add_state = current;
        add_state.last_dir = new_dir;
        let mut sub_state = add_state;
        for iteration in 0..(skip + take) {
            add_state.pos = add_state.pos + new_dir;
            sub_state.pos = sub_state.pos - new_dir;
            if map.contains(&add_state.pos) {
                add_state.heat_loss += map[add_state.pos];
                if iteration >= skip {
                    todo.push(add_state);
                }
            }
            if map.contains(&sub_state.pos) {
                sub_state.heat_loss += map[sub_state.pos];
                if iteration >= skip {
                    todo.push(sub_state);
                }
            }
        }
    }

    todo.pop().unwrap().heat_loss
}

fn main() {
    let map = Map::from_str(&fs::read_to_string(Path::new("data/input17.txt")).unwrap());

    println!("{}", solve(&map, 0, 3));
    println!("{}", solve(&map, 3, 7));
}
