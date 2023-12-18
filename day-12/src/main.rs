use std::{collections::HashMap, env, num::ParseIntError};

use anyhow::{Context, Result};
use aoc_input_lib::get_puzzle_input;
use clap::Parser;

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

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Line {
    springs: Vec<char>,
    groups: Vec<usize>,
}

impl Line {
    fn new(springs: Vec<char>, groups: Vec<usize>) -> Self {
        Self { springs, groups }
    }

    fn parse(line: &str) -> Result<Line> {
        let mut split = line.split_ascii_whitespace();
        let springs: Vec<char> = split.next().unwrap().chars().collect();
        let groups: Vec<usize> = split
            .next()
            .unwrap()
            .split(',')
            .map(|v| usize::from_str_radix(v, 10))
            .collect::<Result<Vec<_>, ParseIntError>>()?;

        Ok(Line::new(springs, groups))
    }

    fn expand(&self) -> Line {
        let springs = self
            .springs
            .iter()
            .cloned()
            .chain(['?'].into_iter())
            .cycle()
            .take(self.springs.len() * 5 + 4)
            .collect();
        let groups = self
            .groups
            .iter()
            .cloned()
            .cycle()
            .take(self.groups.len() * 5)
            .collect();
        Line::new(springs, groups)
    }
}

fn possible_solutions(map: &mut HashMap<Line, usize>, line: &Line) -> usize {
    if let Some(&v) = map.get(line) {
        return v;
    }
    if line.groups.is_empty() {
        let v = match line.springs.iter().any(|c| *c == '#') {
            true => 0,
            false => 1,
        };
        map.insert(line.clone(), v);
        return v;
    }
    if line.springs.len() < line.groups.iter().sum::<usize>() + line.groups.len() - 1 {
        map.insert(line.clone(), 0);
        return 0;
    }
    if line.springs[0] == '.' {
        let s = possible_solutions(
            map,
            &Line::new(line.springs[1..].to_vec(), line.groups.clone()),
        );
        map.insert(line.clone(), s);
        return s;
    }

    let mut solutions = 0;
    let cur = line.groups[0];
    let all_non_operational = line.springs[0..cur].iter().all(|c| *c != '.');
    let end = (cur + 1).min(line.springs.len());
    if all_non_operational
        && ((line.springs.len() > cur && line.springs[cur] != '#') || line.springs.len() <= cur)
    {
        solutions = possible_solutions(
            map,
            &Line::new(line.springs[end..].to_vec(), line.groups[1..].to_vec()),
        );
    }

    if line.springs[0] == '?' {
        solutions += possible_solutions(
            map,
            &Line::new(line.springs[1..].to_vec(), line.groups.clone()),
        );
    }

    map.insert(line.clone(), solutions);
    solutions
}

fn main() -> Result<()> {
    let args = Args::parse();
    let session = args.session.or_else(|| env::var("SESSION").ok());
    let input = if let Some(s) = args.input {
        s
    } else {
        get_puzzle_input(2023, 12, session).context("Could not retrieve puzzle input!")?
    };

    let mut lines = Vec::new();

    for line in input.lines() {
        let l = Line::parse(line)?;
        lines.push(l);
    }

    // p1
    let mut memo = HashMap::new();
    let solutions = lines
        .iter()
        .map(|r| possible_solutions(&mut memo, r))
        .sum::<usize>();

    println!("Part one {}", solutions);

    // p1
    let mut memo = HashMap::new();
    let solutions = lines
        .iter()
        .map(|r| possible_solutions(&mut memo, &r.expand()))
        .sum::<usize>();

    println!("Part two {}", solutions);
    Ok(())
}
