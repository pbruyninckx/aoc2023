use anyhow::Error;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;
use std::ops::Sub;
use std::path::Path;

#[derive(Copy, Clone)]
struct Location {
    row: i64,
    col: i64,
}
struct Image {
    galaxies: Vec<Location>,
    empty_rows: Vec<i64>,
    empty_cols: Vec<i64>,
}

impl Image {
    fn from_str(input: &str) -> Self {
        let mut galaxies: Vec<Location> = vec![];
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push(Location {
                        row: row as i64,
                        col: col as i64,
                    });
                }
            }
        }

        let non_empty_rows: HashSet<i64> = galaxies.iter().map(|l| l.row).collect();
        let mut empty_rows: Vec<i64> = HashSet::from_iter(0..input.lines().count() as i64)
            .sub(&non_empty_rows)
            .into_iter()
            .collect();
        empty_rows.sort();

        let non_empty_cols: HashSet<i64> = galaxies.iter().map(|l| l.col).collect();
        let mut empty_cols: Vec<i64> =
            HashSet::from_iter(0..input.lines().last().unwrap().chars().count() as i64)
                .sub(&non_empty_cols)
                .into_iter()
                .collect();
        empty_cols.sort();

        Self {
            galaxies,
            empty_rows,
            empty_cols,
        }
    }

    fn distance(&self, a: &Location, b: &Location) -> i64 {
        let min_row = min(a.row, b.row);
        let max_row = max(a.row, b.row);
        let min_col = min(a.col, b.col);
        let max_col = max(a.col, b.col);
        max_row - min_row + max_col - min_col
            + self
                .empty_rows
                .iter()
                .filter(|r| min_row < **r && **r < max_row)
                .count() as i64
            + self
                .empty_cols
                .iter()
                .filter(|r| min_col < **r && **r < max_col)
                .count() as i64
    }
}

fn solve(image: &Image) -> i64 {
    let mut ret = 0;
    for i in 0..image.galaxies.len() {
        for j in i + 1..image.galaxies.len() {
            ret += image.distance(&image.galaxies[i], &image.galaxies[j]);
        }
    }

    ret
}

fn main() -> Result<(), Error> {
    let image = Image::from_str(&fs::read_to_string(Path::new("data/input11.txt"))?);
    println!("{}", solve(&image));

    Ok(())
}
