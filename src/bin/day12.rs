use std::collections::HashMap;
use std::fs;
use std::path::Path;

struct Row {
    condition: Vec<char>,
    groups: Vec<usize>,
}

impl Row {
    fn unfold(&self) -> Self {
        Self {
            condition: self
                .condition
                .iter()
                .chain(['?'].iter())
                .cycle()
                .take((self.condition.len() + 1) * 5 - 1)
                .copied()
                .collect(),
            groups: self
                .groups
                .iter()
                .cycle()
                .take(self.groups.len() * 5)
                .copied()
                .collect(),
        }
    }
}

fn parse_line(line: &str) -> Row {
    let (condition, groups) = line.split_once(' ').unwrap();
    Row {
        condition: condition.chars().collect(),
        groups: groups
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect(),
    }
}

fn parse_input(input: &str) -> Vec<Row> {
    input.lines().map(parse_line).collect()
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct SolveState {
    pos: usize,
    num_groups: usize,
}

fn solve_row(row: &Row) -> usize {
    let mut states = HashMap::from([(
        SolveState {
            pos: 0,
            num_groups: 0,
        },
        1_usize,
    )]);
    let num_total_groups = row.groups.len();

    for p in 0..row.condition.len() {
        if row.condition[p] != '#' {
            for g in 0..num_total_groups + 1 {
                if let Some(&num_perms) = states.get(&SolveState {
                    pos: p,
                    num_groups: g,
                }) {
                    states
                        .entry(SolveState {
                            pos: p + 1,
                            num_groups: g,
                        })
                        .and_modify(|n| *n += num_perms)
                        .or_insert(num_perms);
                }
            }
        }

        for g in 0..num_total_groups {
            let end_g = p + row.groups[g];
            if end_g <= row.condition.len()
                && !row.condition[p..end_g].contains(&'.')
                && (end_g == row.condition.len() || row.condition[end_g] != '#')
            {
                if let Some(&num_perms) = states.get(&SolveState {
                    pos: p,
                    num_groups: g,
                }) {
                    states
                        .entry(SolveState {
                            pos: end_g + 1,
                            num_groups: g + 1,
                        })
                        .and_modify(|n| *n += num_perms)
                        .or_insert(num_perms);
                }
            }
        }
    }

    (row.condition.len()..row.condition.len() + 2)
        .filter_map(|p| {
            states.get(&SolveState {
                pos: p,
                num_groups: num_total_groups,
            })
        })
        .sum()
}

fn solve(input: &[Row]) -> usize {
    input.iter().map(solve_row).sum()
}

fn main() {
    let input = parse_input(&fs::read_to_string(Path::new("data/input12.txt")).unwrap());
    println!("{}", solve(&input));
    println!(
        "{}",
        solve(&input.iter().map(Row::unfold).collect::<Vec<Row>>())
    );
}
