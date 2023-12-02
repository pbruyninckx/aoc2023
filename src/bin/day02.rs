use anyhow::Error;
use std::fs::read_to_string;
use std::path::Path;

struct Game {
    id: u32,
    grabs: Vec<Grab>,
}

struct Grab {
    red: u32,
    green: u32,
    blue: u32,
}

impl Grab {
    fn empty() -> Self {
        Grab {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    fn set(&mut self, colour: &str, num: u32) {
        match colour {
            "red" => self.red += num,
            "green" => self.green += num,
            "blue" => self.blue += num,
            _ => panic!(),
        }
    }
}

fn parse_grab(grab_string: &str) -> Grab {
    let mut ret = Grab::empty();
    for (num, colour) in grab_string.split(", ").map(|s| s.split_once(' ').unwrap()) {
        let num = num.parse::<u32>().unwrap();
        ret.set(colour, num);
    }
    ret
}
fn parse_game(line: &str) -> Game {
    let (id_string, grabs_string) = line.split_once(": ").unwrap();
    let id = id_string.split_once(' ').unwrap().1.parse::<u32>().unwrap();
    let grabs = grabs_string.split("; ").map(parse_grab).collect();

    Game { id, grabs }
}

fn valid_game(game: &Game) -> bool {
    game.grabs
        .iter()
        .all(|g| g.red <= 12 && g.green <= 13 && g.blue <= 14)
}
fn solve(games: &[Game]) -> u32 {
    games.iter().filter(|g| valid_game(g)).map(|g| g.id).sum()
}
fn main() -> Result<(), Error> {
    let input: Vec<_> = read_to_string(Path::new("data/input02.txt"))?
        .lines()
        .map(parse_game)
        .collect();
    println!("{}", solve(&input));
    Ok(())
}
