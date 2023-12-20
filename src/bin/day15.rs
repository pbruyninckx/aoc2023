use std::fs;
use std::path::Path;

fn hash(step: &str) -> usize {
    step.chars()
        .fold(0, |ret, c| ((ret + (c as usize)) * 17) % 256)
}

fn solve(sequence: &[&str]) -> usize {
    sequence.iter().map(|&s| hash(s)).sum()
}
fn main() {
    let raw_input = fs::read_to_string(Path::new("data/input15.txt")).unwrap();
    let input: Vec<_> = raw_input.trim().split(',').collect();

    println!("{}", solve(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }
}
