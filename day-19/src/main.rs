mod args;
use args::Args;

use std::{
    collections::{HashMap, VecDeque},
    env,
};

use anyhow::{Context, Result};
use aoc_input_lib::get_puzzle_input;
use clap::Parser;
use phf::{phf_map, Map};
use regex::Regex;
use Op::*;

static MAP: Map<&'static str, usize> = phf_map! {
    "x" => 0,
    "m" => 1,
    "a" => 2,
    "s" => 3,
};

fn main() -> Result<()> {
    let args = Args::parse();
    let session = args.session.or_else(|| env::var("SESSION").ok());

    let input = if let Some(s) = args.input {
        s
    } else {
        get_puzzle_input(2023, 19, session).context("Could not retrieve puzzle input")?
    };

    let re = Regex::new(r"^(\w+)\{(.*)\}$").unwrap();
    let flows: HashMap<String, WorkFlow> = input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|l| WorkFlow::from_str(l, &re))
        .collect();

    let parts: Vec<_> = input
        .split("\n\n")
        .skip(1)
        .next()
        .unwrap()
        .lines()
        .map(Part::from_str)
        .collect();

    part_one_vm(&flows, &parts);
    part_two_vm(&flows);

    Ok(())
}

fn part_one_vm(flows: &HashMap<String, WorkFlow>, parts: &[Part]) {
    let mut sum_part_one = 0;
    'outer: for part in parts.iter() {
        let mut current_flow = flows.get("in").unwrap();
        loop {
            for rule in current_flow.rules.iter() {
                if let Some(dest) = f(rule, part) {
                    match dest {
                        "A" => {
                            sum_part_one += part.rating();
                            continue 'outer;
                        }
                        "R" => continue 'outer,
                        c => {
                            current_flow = flows.get(c).unwrap();
                            break;
                        }
                    }
                }
            }
        }
    }
    println!("{sum_part_one}");
}

fn f<'a>(rule: &'a Rule, part: &Part) -> Option<&'a str> {
    match rule.op {
        Lt => {
            if part.get(rule) < rule.value {
                return Some(&rule.destination);
            }
        }
        Gt => {
            if part.get(rule) > rule.value {
                return Some(&rule.destination);
            }
        }
        None => return Some(&rule.destination),
    }
    Option::None
}

fn part_two_vm(flows: &HashMap<String, WorkFlow>) {
    let mut queue = VecDeque::from(vec![("in", PartRange::new())]);
    let mut accepted = vec![];

    while let Some((dest, range)) = queue.pop_front() {
        let Some(flow) = flows.get(dest) else {
            if dest == "A" {
                accepted.push(range)
            }
            continue;
        };
        for (dest, range) in f2(flow, range) {
            queue.push_back((dest, range));
        }
    }
    let answer = accepted.iter().map(PartRange::product).sum::<u64>();

    println!("{answer}");
}

fn f2<'a>(flow: &'a WorkFlow, mut range: PartRange) -> impl Iterator<Item = (&'a str, PartRange)> {
    // the ranges already processed
    let mut next = vec![];

    // process rules in order
    for rule in flow.rules.iter() {
        if rule.op == None {
            next.push((rule.destination.as_str(), range.clone()));
            continue;
        }

        let prop_range = range.get(rule);
        let (keep, send) = if rule.op == Gt {
            split_range(prop_range, rule.value + 1)
        } else {
            let (l, r) = split_range(prop_range, rule.value);
            // we want to keep the right part, and send the left part, swap tuple
            (r, l)
        };

        if send.0 < send.1 {
            let mut send_copy = range.clone();
            *send_copy.get_mut(rule) = send;
            next.push((rule.destination.as_str(), send_copy));
        }
        if keep.0 < keep.1 {
            *range.get_mut(rule) = keep;
        }
    }

    next.into_iter()
}

fn split_range((left, right): (i32, i32), value: i32) -> ((i32, i32), (i32, i32)) {
    ((left, value), (value, right))
}

#[derive(Debug, Clone)]
struct WorkFlow {
    rules: Vec<Rule>,
}

impl WorkFlow {
    fn from_str(v: &str, re: &Regex) -> (String, Self) {
        let (_m, [id, rules]) = re.captures(v).map(|c| c.extract()).unwrap();
        let rules: Vec<_> = rules.split(',').map(Rule::from_str).collect();
        (id.into(), Self { rules })
    }
}

/// Represents a single rule in a workflow
#[derive(Debug, Clone)]
struct Rule {
    prop: String,
    op: Op,
    value: i32,
    destination: String,
}

impl Rule {
    fn from_str(v: &str) -> Self {
        // no op
        if !v.contains(':') {
            return Self {
                prop: ".".into(),
                op: None,
                value: 0,
                destination: v.into(),
            };
        }
        let v: Vec<_> = v.split(':').collect();

        let op = match &v[0][1..2] {
            ">" => Gt,
            "<" => Lt,
            c => panic!("expected < or >, got {c}"),
        };
        Self {
            prop: v[0][0..1].into(),
            op,
            value: i32::from_str_radix(&v[0][2..], 10).unwrap(),
            destination: v[1].into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Lt,
    Gt,
    None,
}

#[derive(Debug, Clone, Copy)]
struct Part {
    xmas: [i32; 4],
}

impl Part {
    fn from_str(v: &str) -> Self {
        let mut xmas = [0; 4];
        // remove '{'  and '}' delimiters
        let v = &v[1..v.len() - 1];
        for (i, v) in v.split(',').enumerate() {
            let value = i32::from_str_radix(v.split('=').skip(1).next().unwrap(), 10).unwrap();
            xmas[i] = value;
        }
        Self { xmas }
    }

    fn get(&self, rule: &Rule) -> i32 {
        self.xmas[*MAP.get(&rule.prop).unwrap()]
    }

    fn rating(&self) -> i32 {
        self.xmas.iter().sum()
    }
}

#[derive(Debug, Clone)]
struct PartRange {
    xmas: [(i32, i32); 4],
}

impl PartRange {
    fn new() -> Self {
        Self {
            xmas: [(1, 4001); 4],
        }
    }

    fn get(&self, rule: &Rule) -> (i32, i32) {
        self.xmas[*MAP.get(&rule.prop).unwrap()]
    }

    fn get_mut(&mut self, rule: &Rule) -> &mut (i32, i32) {
        &mut self.xmas[*MAP.get(&rule.prop).unwrap()]
    }

    fn is_empty(&self) -> bool {
        // left inclusive, right exclusive
        // so 1..1 = empty
        self.xmas.iter().any(|(l, r)| l >= r)
    }

    fn product(&self) -> u64 {
        assert!(!self.is_empty());
        self.xmas
            .iter()
            .map(|(left, right)| (*right as u64) - (*left as u64))
            .product()
    }
}
