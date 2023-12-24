use itertools::izip;
use std::fs;
use std::ops::AddAssign;
use std::path::Path;

#[derive(Clone)]
struct Instruction {
    direction: char,
    distance: i64,
    color: String,
}

impl Instruction {
    fn correct(&self) -> Self {
        let chars = self.color.chars();
        let direction = match chars.clone().last().unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => panic!("Invalid input"),
        };
        let distance = i64::from_str_radix(&chars.take(5).collect::<String>(), 16).unwrap();
        Self {
            direction,
            distance,
            color: String::new(),
        }
    }
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
            color: parts[2]
                .chars()
                .filter(|c| c.is_ascii_alphanumeric())
                .collect(),
        }
    }
}

fn parse_input(string: &str) -> Vec<Instruction> {
    string.lines().map(Instruction::from_str).collect()
}

fn solve(instructions: &Vec<Instruction>, transform: &dyn Fn(&Instruction) -> Instruction) -> i64 {
    struct State {
        height: i64,
        area: i64,
    }

    let extended_it = instructions
        .iter()
        .map(transform)
        .cycle()
        .take(instructions.len() + 2);
    izip!(
        extended_it.clone(),
        extended_it.clone().skip(1),
        extended_it.skip(2)
    )
    .filter(|(_, instr, _)| ['L', 'R'].contains(&instr.direction))
    .fold(
        State { height: 0, area: 0 },
        |mut state, (before, instr, after)| {
            state.height += before.distance * (if before.direction == 'U' { 1 } else { -1 });
            state.area += match (before.direction, instr.direction, after.direction) {
                ('U', 'R', 'D') => (instr.distance + 1) * state.height,
                ('D', 'R', 'D') | ('U', 'R', 'U') => instr.distance * state.height,
                ('D', 'R', 'U') => (instr.distance - 1) * state.height,
                ('U', 'L', 'D') => -(instr.distance - 1) * (state.height - 1),
                ('U', 'L', 'U') | ('D', 'L', 'D') => -instr.distance * (state.height - 1),
                ('D', 'L', 'U') => -(instr.distance + 1) * (state.height - 1),
                (_, _, _) => panic!("Not expecting this"),
            };
            state
        },
    )
    .area
}

fn main() {
    let instructions = parse_input(&fs::read_to_string(Path::new("data/input18.txt")).unwrap());

    println!("{}", solve(&instructions, &Instruction::clone));
    println!("{}", solve(&instructions, &Instruction::correct));
}
