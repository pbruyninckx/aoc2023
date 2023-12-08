use anyhow::Error;
use aoc2023::parse_numbers;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug)]
struct Range {
    start: i64,
    length: i64,
}

struct MapRange {
    dest_start: i64,
    source_start: i64,
    length: i64,
}

struct MappedRange {
    unmapped: Vec<Range>,
    mapped: Option<Range>,
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

    fn apply(&self, value: i64) -> Option<i64> {
        if self.source_start <= value && value - self.source_start < self.length {
            Some(self.dest_start + (value - self.source_start))
        } else {
            None
        }
    }

    fn apply_range(&self, range: &Range) -> MappedRange {
        let range_end = range.start + range.length;
        let source_end = self.source_start + self.length;
        let dest_end = self.dest_start + self.length;

        if range_end <= self.source_start || range.start >= source_end {
            MappedRange {
                mapped: None,
                unmapped: vec![range.clone()],
            }
        } else {
            let mut unmapped = vec![];
            if range.start < self.source_start {
                unmapped.push(Range {
                    start: range.start,
                    length: self.source_start - range.start,
                });
            }
            if range_end > source_end {
                unmapped.push(Range {
                    start: source_end,
                    length: range_end - source_end,
                })
            }
            let mapped_start = self.apply(range.start).unwrap_or(self.dest_start);
            let mapped_end = self.apply(range_end).unwrap_or(dest_end);
            let mapped = Some(Range {
                start: mapped_start,
                length: mapped_end - mapped_start,
            });
            MappedRange { mapped, unmapped }
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

    fn apply(&self, number: i64) -> i64 {
        let ret = self
            .ranges
            .iter()
            .filter_map(|range| range.apply(number))
            .next()
            .unwrap_or(number);
        ret
    }

    fn apply_range(&self, range: &Range) -> Vec<Range> {
        let mut ret = vec![];
        let mut to_map = vec![range.clone()];
        for map_range in &self.ranges {
            let mut to_map_next: Vec<Range> = vec![];
            for range_to_map in &to_map {
                let mut half_mapped = map_range.apply_range(range_to_map);
                to_map_next.append(&mut half_mapped.unmapped);
                if let Some(mapped) = half_mapped.mapped {
                    ret.push(mapped);
                }
            }
            to_map = to_map_next;
        }

        ret.append(&mut to_map);
        ret
    }

    fn apply_ranges(&self, ranges: &[Range]) -> Vec<Range> {
        ranges.iter().flat_map(|r| self.apply_range(r)).collect()
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

    fn seed_ranges(&self) -> Vec<Range> {
        let mut ret = vec![];
        let mut it = self.seeds.iter().peekable();
        while it.peek().is_some() {
            let start = *it.next().unwrap();
            let length = *it.next().unwrap();
            ret.push(Range { start, length });
        }
        ret
    }
}

fn solve(almanac: &Almanac) -> i64 {
    almanac
        .seeds
        .iter()
        .map(|seed| {
            almanac
                .maps
                .iter()
                .fold(*seed, |number, mapping| mapping.apply(number))
        })
        .min()
        .unwrap()
}

fn solve2(almanac: &Almanac) -> i64 {
    almanac
        .maps
        .iter()
        .fold(almanac.seed_ranges(), |ranges, mapping| {
            mapping.apply_ranges(&ranges)
        })
        .iter()
        .map(|r| r.start)
        .min()
        .unwrap()
}

fn main() -> Result<(), Error> {
    let almanac = Almanac::from_lines(
        &fs::read_to_string(Path::new("data/input05.txt"))?
            .lines()
            .collect::<Vec<_>>(),
    )?;
    println!("{}", solve(&almanac));
    println!("{}", solve2(&almanac));
    Ok(())
}
