use anyhow::Error;

pub fn parse_numbers(string: &str) -> Result<Vec<i64>, Error> {
    Ok(string
        .trim()
        .split_ascii_whitespace()
        .map(|d| d.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?)
}
