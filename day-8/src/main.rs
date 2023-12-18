use std::{collections::HashMap, env};

use anyhow::{Context, Result};
use aoc_input_lib::get_puzzle_input;
use clap::Parser;
use regex::Regex;
use num::integer::lcm;
use num::BigInt;


/// Wait for it
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
        get_puzzle_input(2023, 8, session).context("Could not retrieve puzzle input!")?
    };

    let re = Regex::new(r"^(.*) = \((.*), (.*)\)")?;
    let mut nodes_map = HashMap::new();

    for line in input.lines().skip(2) {
        let (_m, [parent, left, right]) = re.captures(&line).context("should match")?.extract();
        nodes_map.insert(parent, (left, right));
    }

    let instructions = input.lines().next().unwrap();
    let mut root = "AAA";

    let mut steps = 0;
    for ch in instructions.chars().cycle() {
        if root == "ZZZ" {
            break;
        }
        steps += 1;
        match ch {
            'L' => root = nodes_map.get(root).unwrap().0,
            'R' => root = nodes_map.get(root).unwrap().1,
            _ => panic!("unexpected input"),
        }
    }
    println!("Part one: {}", steps);

    let root_nodes: Vec<_> = nodes_map
        .clone()
        .into_keys()
        .filter(|v| v.chars().last().is_some_and(|ch| ch == 'A'))
        .collect();
    let mut steps_for_each_root = Vec::new();
    for mut root in root_nodes {
        let mut steps: u64 = 0;
        for ch in instructions.chars().cycle() {
            if root.as_bytes()[2] == b'Z' {
                break;
            }
            steps += 1;
            match ch {
                'L' => root = nodes_map.get(root).unwrap().0,
                'R' => root = nodes_map.get(root).unwrap().1,
                _ => panic!("unexpected input"),
            }
        }
        steps_for_each_root.push(steps);
    }
    dbg!(&steps_for_each_root);
    let bigs = steps_for_each_root.into_iter().map(|n| BigInt::from(n)).reduce(|a,b| lcm(a, b)).unwrap();
    println!("Part two: {}", bigs);
    Ok(())
}
