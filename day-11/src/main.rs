use std::env;

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

fn main() -> Result<()> {
    let args = Args::parse();
    let session = args.session.or_else(|| env::var("SESSION").ok());
    let input = if let Some(s) = args.input {
        s
    } else {
        get_puzzle_input(2023, 11, session).context("Could not retrieve puzzle input!")?
    };

    let mut universe: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    //
    // expand universe horizontally
    //
    let empty_rows: Vec<_> = universe
        .iter()
        .enumerate()
        .filter_map(|(idx, line)| line.iter().all(|ch| ch == &'.').then_some(idx))
        .collect();

    for (row, offset_from_copies) in empty_rows.clone().into_iter().enumerate() {
        let copy = universe[row + offset_from_copies].clone();
        universe.insert(row + offset_from_copies, copy);
    }

    //
    // expand universe vertically
    //
    let n_cols = universe.first().unwrap().len();
    let empty_cols: Vec<_> = (0..n_cols)
        .filter_map(|c| {
            universe
                .iter()
                .map(move |line| line[c])
                .all(|ch| ch == '.')
                .then_some(c)
        })
        .collect();

    for (col, offset_from_copies) in empty_cols.clone().into_iter().enumerate() {
        for line in universe.iter_mut() {
            line.insert(col + offset_from_copies, '.')
        }
    }

    //
    // build map of galaxies and their location
    // e.g. galaxies: (id, (col,row))
    //
    let n_cols = universe.first().unwrap().len();
    let n_rows = universe.len();
    let mut galaxies = Vec::new();
    for row in 0..n_rows {
        for col in 0..n_cols {
            if universe[row][col] == '#' {
                galaxies.push((row, col));
            }
        }
    }

    // compare pairs of galaxies
    let mut distances = Vec::new();
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            // skip 'same' pair
            if i == j {
                continue;
            }
            // compute manhatten distance between galaxies
            let (x1, y1) = galaxies[i];
            let (x2, y2) = galaxies[j];
            let dist = abs(x2, x1) + abs(y2, y1);
            distances.push(dist);
        }
    }
    println!("Part one: {}", distances.iter().sum::<usize>());

    //
    // part two
    //
    // idea is the same;
    // - compare every pair in the unexpanded universe
    // - count the times we cross an empty column or row
    // - multiply the distance by this count
    // - probably use a big number type

    // parse
    let universe: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    //
    // array of galaxies' location
    //
    let n_cols = universe.first().unwrap().len();
    let n_rows = universe.len();
    let mut galaxies = Vec::new();
    for row in 0..n_rows {
        for col in 0..n_cols {
            if universe[row][col] == '#' {
                galaxies.push((row as u128, col as u128));
            }
        }
    }

    // compare pairs of galaxies
    let mut distances = Vec::new();

    let empty_rows = empty_rows
        .into_iter()
        .map(|x| x as u128)
        .collect::<Vec<_>>();
    let empty_cols = empty_cols
        .into_iter()
        .map(|x| x as u128)
        .collect::<Vec<_>>();

    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            // skip 'same' pair
            if i == j {
                continue;
            }
            // compute manhatten distance between galaxies
            let (x1, y1) = galaxies[i];
            let (x2, y2) = galaxies[j];
            let (x_max, x_min) = (u128::max(x1, x2), u128::min(x1, x2));
            let (y_max, y_min) = (u128::max(y1, y2), u128::min(y1, y2));
            let dist = (x_max - x_min) + (y_max - y_min);

            let mut times_crossed = 0;
            for row in empty_rows.iter().copied() {
                if x_min < row && row < x_max {
                    times_crossed += 1;
                }
            }
            for col in empty_cols.iter().copied() {
                if y_min < col && col < y_max {
                    times_crossed += 1;
                }
            }

            // correct for the times we crossed that huge distance
            let dist = dist + times_crossed * 1_000_000 - times_crossed;
            distances.push(dist);
        }
    }
    println!("Part two: {}", distances.iter().sum::<u128>());

    Ok(())
}

fn abs(a: usize, b: usize) -> usize {
    return if b >= a { b - a } else { a - b };
}
