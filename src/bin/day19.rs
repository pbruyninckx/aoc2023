use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn from_str(string: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[0-9]+").unwrap();
        }
        let matches: Vec<i64> = RE
            .find_iter(string)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        Self {
            x: matches[0],
            m: matches[1],
            a: matches[2],
            s: matches[3],
        }
    }

    fn selector_func(v: &str) -> fn(&Part) -> i64 {
        match v {
            "x" => |p| p.x,
            "m" => |p| p.m,
            "a" => |p| p.a,
            "s" => |p| p.s,
            _ => panic!("Invalid input"),
        }
    }

    fn rating(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone)]
enum Target {
    Workflow(String),
    Accepted,
    Rejected,
}

impl Target {
    fn from_str(string: &str) -> Self {
        match string {
            "A" => Target::Accepted,
            "R" => Target::Rejected,
            _ => Target::Workflow(String::from(string)),
        }
    }
}

struct Rule {
    condition: Box<dyn Fn(&Part) -> bool>,
    target: Target,
}

impl Rule {
    fn from_str(string: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new("([xmas])(.)([0-9]+):(.+)").unwrap();
        }
        if let Some(captures) = RE.captures(string) {
            let selector_fn = Part::selector_func(captures.get(1).unwrap().as_str());
            let comp_fn = match captures.get(2).unwrap().as_str() {
                "<" => i64::lt,
                ">" => i64::gt,
                _ => panic!("Invalid input"),
            };
            let value = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let condition = Box::new(move |part: &Part| comp_fn(&selector_fn(part), &value));
            let target = Target::from_str(captures.get(4).unwrap().as_str());
            Rule { condition, target }
        } else {
            Rule {
                condition: Box::new(|_| true),
                target: Target::from_str(string),
            }
        }
    }
}

struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn process(&self, part: &Part) -> Target {
        for rule in &self.rules {
            if (rule.condition)(part) {
                return rule.target.clone();
            }
        }
        panic!("Not supposed to end up here");
    }
}

type System = HashMap<String, Workflow>;

trait SystemExt {
    fn accept(&self, part: &Part) -> bool;
}

impl SystemExt for System {
    fn accept(&self, part: &Part) -> bool {
        let mut workflow = self.get(&String::from("in")).unwrap();
        loop {
            match workflow.process(part) {
                Target::Workflow(workflow_name) => {
                    workflow = self.get(&workflow_name).unwrap();
                }
                Target::Accepted => {
                    return true;
                }
                Target::Rejected => {
                    return false;
                }
            }
        }
    }
}

fn parse_workflows(string: &str) -> System {
    string
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(&['{', ',', '}']).collect();
            (
                String::from(parts[0]),
                Workflow {
                    rules: parts[1..parts.len() - 1]
                        .iter()
                        .map(|&s| Rule::from_str(s))
                        .collect::<Vec<_>>(),
                },
            )
        })
        .collect()
}

fn parse_input(input: &str) -> (System, Vec<Part>) {
    let (system_str, part_str) = input.split_once("\n\n").unwrap();
    let system = parse_workflows(system_str);
    let parts = part_str.lines().map(Part::from_str).collect();
    (system, parts)
}

fn solve(input: &(System, Vec<Part>)) -> i64 {
    let (system, parts) = input;

    parts
        .iter()
        .filter(|&p| system.accept(p))
        .map(Part::rating)
        .sum()
}

fn main() {
    let input = parse_input(&fs::read_to_string(Path::new("data/input19.txt")).unwrap());

    println!("{}", solve(&input));
}
