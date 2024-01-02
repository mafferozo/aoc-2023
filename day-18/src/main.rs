use std::env;

use ::phf::{phf_map, Map};
use anyhow::{Context, Result};
use aoc_input_lib::get_puzzle_input;
use clap::Parser;

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

static MAP: Map<&'static str, (i64, i64)> = phf_map! {
    "R" => (1,0),
    "D" => (0,1),
    "L" => (-1,0),
    "U" => (0,-1),

    "0" => (1,0),
    "1" => (0,1),
    "2" => (-1,0),
    "3" => (0,-1),
};

fn main() -> Result<()> {
    let args = Args::parse();
    let session = args.session.or_else(|| env::var("SESSION").ok());

    let input = if let Some(s) = args.input {
        s
    } else {
        get_puzzle_input(2023, 18, session).context("Could not retrieve puzzle input")?
    };

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
    Ok(())
}

fn part_one(input: &str) -> i64 {
    let iter = input.lines().map(|l| {
        let mut it = l.split_ascii_whitespace();
        let (x, y) = MAP.get(it.next().unwrap()).unwrap();
        let n = i64::from_str_radix(it.next().unwrap(), 10).unwrap();
        (*x, *y, n)
    });
    f(iter)
}

fn part_two(input: &str) -> i64 {
    let iter = input.lines().map(|l| {
        let color = l.split_ascii_whitespace().skip(2).next().unwrap();

        let n = i64::from_str_radix(&color[2..7], 16).unwrap();
        let (x, y) = MAP.get(&color[7..8]).unwrap();
        (*x, *y, n)
    });
    f(iter)
}

fn f(steps: impl Iterator<Item = (i64, i64, i64)>) -> i64 {
    // first point is at (0,0)
    let (mut pos_x, mut pos_y) = (0, 0);

    let mut boundary_points = 0;
    let mut area = 0;

    for (x, y, n) in steps {
        // position of new point
        let new_pos_x = pos_x + x * n;
        let new_pos_y = pos_y + y * n;

        // computes area of triangle
        area += pos_x * new_pos_y - new_pos_x * pos_y;

        // count all the boundary points too
        boundary_points += n;

        // update position of old point
        pos_x = new_pos_x;
        pos_y = new_pos_y;
    }
    // note: we still need to divide the area by two after the integral
    let area = area / 2;
    // Picks' theorem
    // area = i + b/2 -1
    // i = area - b/2 + 1
    let inside_points = area - boundary_points / 2 + 1;

    inside_points + boundary_points
}
