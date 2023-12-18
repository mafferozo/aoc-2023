use std::{collections::HashMap, env};

use anyhow::{Context, Result};
use aoc_input_lib::get_puzzle_input;
use clap::Parser;
use regex::Regex;

/// Gear Ratios
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
        get_puzzle_input(2023, 3, session).context("Could not retrieve puzzle input")?
    };

    // find table length by finding position of the first newline
    let newline_position = input
        .char_indices()
        .find(|(_pos, c)| c == &'\n')
        .map(|(pos, _c)| pos)
        .context("Puzzle input should contain different lines")?;
    // strip the newlines
    let input = input.replace("\n", "");
    let n_columns = newline_position;
    let len = input.len();
    let n_rows = len / n_columns;

    // A function that computes index from relative neighbour position x_rel,y_rel
    // returns None if relative index lies outside the grid
    let relative_index = |index: usize, x_rel: isize, y_rel: isize| {
        let (x, y) = (index % n_columns, index / n_columns);
        let y = y.checked_add_signed(y_rel);
        let x = x.checked_add_signed(x_rel);
        if x.is_some_and(|x| x < n_columns) && y.is_some_and(|y| y < n_rows) {
            Some(y.unwrap() * n_columns + x.unwrap())
        } else {
            None
        }
    };

    let neighbours = vec![
        // top row
        (-1, -1),
        (0, -1),
        (1, -1),
        // left, right
        (-1, 0),
        (1, 0),
        // bottom row
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut sum_part_one = 0;
    let mut gears: HashMap<usize, Vec<u32>> = HashMap::new();

    // iterate over each number in the grid
    let re = Regex::new(r"\d+").unwrap();
    for m in re.find_iter(&input) {
        let mut symbol = None;
        // for each digit in the number, check for a neighbouring symbol
        for c in m.start()..m.end() {
            symbol = neighbours
                .iter()
                .copied()
                // make sure to filter out neighbours that are off the grid
                .filter_map(|(x, y)| relative_index(c, x, y))
                // map to a single char
                .map(|i| input.char_indices().nth(i).unwrap())
                // returns true if a symbol is found in any neighbour
                .find(|(_i, ch)| !(ch.is_digit(10) || ch == &'.'));

            if symbol.is_some() {
                break;
            }
        }
        if let Some((index, ch)) = symbol {
            let number = u32::from_str_radix(m.as_str(), 10).unwrap();
            sum_part_one += number;
            if ch == '*' {
                // add number to gear index in hashmap
                gears
                    .entry(index)
                    .and_modify(|v| v.push(number))
                    .or_insert(vec![number]);
            }
        }
    }

    let mut sum_part_two = 0;
    for (_k,v) in gears.drain() {
        if v.len() != 2 {
            continue
        }
        sum_part_two += v.into_iter().product::<u32>();
    }

    println!("part one: {}", sum_part_one);
    println!("part two: {}", sum_part_two);

    Ok(())
}
