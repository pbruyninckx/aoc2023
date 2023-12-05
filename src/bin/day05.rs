use anyhow::Error;
use aoc2023::parse_numbers;
use std::fs;
use std::path::Path;

struct MapRange {
    dest_start: i64,
    source_start: i64,
    length: i64,
}

impl MapRange {
    fn from_string(line: &str) -> Option<Self> {
        let numbers = parse_numbers(line).ok()?;
        if numbers.len() == 3 {
            Some(Self {
                dest_start: numbers[0],
                source_start: numbers[1],
                length: numbers[2],
            })
        } else {
            None
        }
    }
}

struct Mapping {
    ranges: Vec<MapRange>,
}

impl Mapping {
    fn from_iter(it: &mut dyn Iterator<Item = &&str>) -> Mapping {
        let mut ranges = vec![];
        it.next();
        for line in it.take_while(|l| !l.is_empty()) {
            ranges.push(MapRange::from_string(line).unwrap());
        }

        Self { ranges }
    }
}
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Mapping>,
}

impl Almanac {
    fn from_lines(lines: &[&str]) -> Result<Self, Error> {
        let seeds = parse_numbers(
            lines
                .first()
                .ok_or(Error::msg("non-empty file expected"))?
                .strip_prefix("seeds: ")
                .ok_or(Error::msg("Seeds expected"))?,
        )?;

        let mut maps = vec![];
        let mut line_iter = lines.iter().skip(2).peekable();
        while line_iter.peek().is_some() {
            maps.push(Mapping::from_iter(&mut line_iter));
        }
        Ok(Self { seeds, maps })
    }
}
fn main() -> Result<(), Error> {
    let almanac = Almanac::from_lines(
        &fs::read_to_string(Path::new("data/input05.txt"))?
            .lines()
            .collect::<Vec<_>>(),
    )?;
    println!("{}", almanac.maps.len());
    Ok(())
}
