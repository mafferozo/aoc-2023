use std::{collections::HashMap, env};

use anyhow::{Context, Result};
use aoc_input_lib::get_puzzle_input;
use clap::Parser;
use regex::Regex;
use Op::*;

/// Lavaduct Lagoon
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The puzzle input
    #[arg()]
    input: Option<String>,

    /// Advent of code session token
    #[arg(short, long)]
    session: Option<String>,
}

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

    let mut sum_part_one = 0;
    'outer: for part in parts.iter() {
        let mut current_flow = flows.get("in").unwrap();
        println!();
        println!();
        loop {
            // dbg!(current_flow, part);
            for rule in current_flow.rules.iter() {
                if let Some(dest) = f(rule, part) {
                    match dest {
                        "A" => {
                            sum_part_one += part.rating();
                            continue 'outer;
                        }
                        "R" => continue 'outer,
                        c => {
                            dbg!(c);
                            current_flow = flows.get(c).unwrap();
                            break;
                        }
                    }
                }
            }
        }
    }
    println!("{sum_part_one}");

    Ok(())
}

fn f<'a>(rule: &'a Rule, part: &Part) -> Option<&'a str> {
    match rule.op {
        Less => match rule.prop.as_str() {
            "x" => {
                if part.x < rule.value {
                    return Some(&rule.destination);
                }
            }
            "m" => {
                if part.m < rule.value {
                    return Some(&rule.destination);
                }
            }
            "a" => {
                if part.a < rule.value {
                    return Some(&rule.destination);
                }
            }
            "s" => {
                if part.s < rule.value {
                    return Some(&rule.destination);
                }
            }
            c => panic!("{} not in xmas", c),
        },
        Greater => match rule.prop.as_str() {
            "x" => {
                if part.x > rule.value {
                    return Some(&rule.destination);
                }
            }
            "m" => {
                if part.m > rule.value {
                    return Some(&rule.destination);
                }
            }
            "a" => {
                if part.a > rule.value {
                    return Some(&rule.destination);
                }
            }
            "s" => {
                if part.s > rule.value {
                    return Some(&rule.destination);
                }
            }
            c => panic!("{} not in xmas", c),
        },
        Noop => return Some(&rule.destination),
    }
    None
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
                op: Noop,
                value: 0,
                destination: v.into(),
            };
        }
        let v: Vec<_> = v.split(':').collect();

        let op = match &v[0][1..2] {
            ">" => Greater,
            "<" => Less,
            _ => Noop,
        };
        Self {
            prop: v[0][0..1].into(),
            op,
            value: i32::from_str_radix(&v[0][2..], 10).unwrap(),
            destination: v[1].into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Less,
    Greater,
    Noop,
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    fn from_str(v: &str) -> Self {
        let v = &v[1..v.len() - 1];
        let mut it = v.split([',', '=']);
        it.next();
        let x = i32::from_str_radix(it.next().unwrap(), 10).unwrap();
        it.next();
        let m = i32::from_str_radix(it.next().unwrap(), 10).unwrap();
        it.next();
        let a = i32::from_str_radix(it.next().unwrap(), 10).unwrap();
        it.next();
        let s = i32::from_str_radix(it.next().unwrap(), 10).unwrap();
        Self { x, m, a, s }
    }

    fn rating(&self) -> i32 {
        let Self { x, m, a, s } = self;
        x + m + a + s
    }
}
