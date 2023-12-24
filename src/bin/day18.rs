use std::collections::HashSet;
use std::fs;
use std::ops::AddAssign;
use std::path::Path;

struct Instruction {
    direction: char,
    distance: i64,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Pos {
    r: i64,
    c: i64,
}

impl AddAssign<char> for Pos {
    fn add_assign(&mut self, rhs: char) {
        match rhs {
            'U' => self.r -= 1,
            'D' => self.r += 1,
            'L' => self.c -= 1,
            'R' => self.c += 1,
            _ => panic!("Wrong input"),
        }
    }
}

impl Instruction {
    fn from_str(string: &str) -> Self {
        let parts: Vec<_> = string.split_ascii_whitespace().collect();
        let direction = parts[0].chars().next().unwrap();
        let distance = parts[1].parse().unwrap();
        Self {
            direction,
            distance,
        }
    }
}

fn parse_input(string: &str) -> Vec<Instruction> {
    string.lines().map(Instruction::from_str).collect()
}

fn solve(instructions: &Vec<Instruction>) -> i64 {
    let edges = get_edges(instructions);

    let up = edges.iter().map(|p| p.r).min().unwrap();
    let down = edges.iter().map(|p| p.r).max().unwrap();
    let left = edges.iter().map(|p| p.c).min().unwrap();
    let right = edges.iter().map(|p| p.c).max().unwrap();

    let edge_set: HashSet<Pos> = HashSet::from_iter(edges);

    let mut ret = 0;
    for r in up..down + 1 {
        let mut active = false;
        let mut from = ' ';
        for c in left..right + 1 {
            if edge_set.contains(&Pos { r, c }) != edge_set.contains(&Pos { r, c: c - 1 }) {
                if edge_set.contains(&Pos { r, c }) {
                    from = if edge_set.contains(&Pos { r: r + 1, c })
                        && edge_set.contains(&Pos { r: r - 1, c })
                    {
                        'B'
                    } else if edge_set.contains(&Pos { r: r + 1, c }) {
                        'D'
                    } else {
                        assert!(edge_set.contains(&Pos { r: r - 1, c }));
                        'U'
                    }
                } else if from == 'B'
                    || edge_set.contains(&Pos { r: r + 1, c: c - 1 }) == (from == 'U')
                {
                    active = !active;
                }
            }
            if edge_set.contains(&Pos { r, c }) || active {
                ret += 1;
            }
        }
    }

    ret
}

fn get_edges(instructions: &Vec<Instruction>) -> Vec<Pos> {
    let mut edges = vec![Pos { r: 0, c: 0 }];
    for instruction in instructions {
        let mut pos = *edges.last().unwrap();
        for _ in 0..instruction.distance {
            pos += instruction.direction;
            edges.push(pos);
        }
    }
    assert_eq!(edges[0], *edges.last().unwrap());
    edges.pop();
    edges
}

fn main() {
    let instructions = parse_input(&fs::read_to_string(Path::new("data/input18.txt")).unwrap());

    println!("{}", solve(&instructions));
}
