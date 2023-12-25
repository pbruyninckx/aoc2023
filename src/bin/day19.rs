use lazy_static::lazy_static;
use num::clamp;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy)]
struct Range {
    begin: i64,
    end: i64,
}

impl Range {
    fn split(&self, value: i64, comp: &str) -> (Self, Self) {
        match comp {
            "<" => (
                Range {
                    begin: self.begin,
                    end: clamp(value, self.begin, self.end),
                },
                Range {
                    begin: clamp(value, self.begin, self.end),
                    end: self.end,
                },
            ),
            ">" => (
                Range {
                    begin: clamp(value + 1, self.begin, self.end),
                    end: self.end,
                },
                Range {
                    begin: self.begin,
                    end: clamp(value + 1, self.begin, self.end),
                },
            ),
            _ => panic!("Invalid input"),
        }
    }
}

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

struct ConditionParams {
    var: String,
    comp: String,
    value: i64,
}

struct Rule {
    condition: Box<dyn Fn(&Part) -> bool>,
    target: Target,
    condition_params: Option<ConditionParams>,
}

impl Rule {
    fn from_str(string: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new("([xmas])(.)([0-9]+):(.+)").unwrap();
        }
        if let Some(captures) = RE.captures(string) {
            let var = captures.get(1).unwrap().as_str();
            let selector_fn = Part::selector_func(var);
            let comp = captures.get(2).unwrap().as_str();
            let comp_fn = match comp {
                "<" => i64::lt,
                ">" => i64::gt,
                _ => panic!("Invalid input"),
            };
            let value = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let condition = Box::new(move |part: &Part| comp_fn(&selector_fn(part), &value));
            let target = Target::from_str(captures.get(4).unwrap().as_str());
            let condition_params = Some(ConditionParams {
                var: String::from(var),
                comp: String::from(comp),
                value,
            });
            Rule {
                condition,
                target,
                condition_params,
            }
        } else {
            Rule {
                condition: Box::new(|_| true),
                target: Target::from_str(string),
                condition_params: None,
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

type PartRange = HashMap<String, Range>;

trait PartRangeExt {
    fn split(&self, condition: &ConditionParams) -> (PartRange, PartRange);
    fn new_xmas() -> Self;
    fn num_combinations(&self) -> i64;
}

impl PartRangeExt for PartRange {
    fn split(&self, condition: &ConditionParams) -> (PartRange, PartRange) {
        let mut ret1 = self.clone();
        let mut ret2 = self.clone();

        let (range1, range2) = self
            .get(&condition.var)
            .unwrap()
            .split(condition.value, &condition.comp);
        ret1.entry(condition.var.clone())
            .and_modify(move |r| *r = range1);
        ret2.entry(condition.var.clone())
            .and_modify(move |r| *r = range2);

        (ret1, ret2)
    }

    fn new_xmas() -> Self {
        "xmas"
            .chars()
            .map(|c| {
                (
                    String::from(c),
                    Range {
                        begin: 1,
                        end: 4001,
                    },
                )
            })
            .collect()
    }

    fn num_combinations(&self) -> i64 {
        self.values().map(|r| r.end - r.begin).product()
    }
}

type System = HashMap<String, Workflow>;

trait SystemExt {
    fn accept(&self, part: &Part) -> bool;

    fn num_combinations(&self, rule: &str, part_range: &PartRange) -> i64;
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

    fn num_combinations(&self, rule: &str, part_range: &PartRange) -> i64 {
        let rules = &self.get(rule).unwrap().rules;
        let mut ret = 0;

        let mut part_range = part_range.clone();
        for rule in rules {
            if let Some(condition_parameters) = &rule.condition_params {
                let (this_part_range, next_part_range) = part_range.split(condition_parameters);
                part_range = next_part_range;
                ret += match &rule.target {
                    Target::Workflow(workflow) => self.num_combinations(workflow, &this_part_range),
                    Target::Accepted => this_part_range.num_combinations(),
                    Target::Rejected => 0,
                }
            } else {
                ret += match &rule.target {
                    Target::Workflow(workflow) => self.num_combinations(workflow, &part_range),
                    Target::Accepted => part_range.num_combinations(),
                    Target::Rejected => 0,
                }
            }
        }

        ret
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

fn solve2(system: &System) -> i64 {
    system.num_combinations("in", &PartRange::new_xmas())
}

fn main() {
    let input = parse_input(&fs::read_to_string(Path::new("data/input19.txt")).unwrap());

    println!("{}", solve(&input));
    println!("{}", solve2(&input.0));
}
