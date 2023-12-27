use std::{env, fmt::{Display, Debug}};

use anyhow::{Context, Result};
use aoc_input_lib::get_puzzle_input;
use clap::Parser;

/// Parabolic Reflector Dish
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
        get_puzzle_input(2023, 14, session).context("Could not retrieve puzzle input!")?
    };
    let mut p = Platform::parse_platform(&input);
    p.tilt_north();
    println!("Part one: {}", p.compute_total_load());


    let mut p = Platform::parse_platform(&input);
    let mut loads = vec![];
    // 500 computations should be enough to detect the cycle
    for _ in 0..500 {
        p.tilt_north();
        p.tilt_west();
        p.tilt_south();
        p.tilt_east();
        loads.push(p.compute_total_load())
    }

    let (cycle_start,length_of_cycle) = detect_cycle(&loads).unwrap();
    let remainder = (1_000_000_000-cycle_start) % length_of_cycle;
    let part_two = &loads[cycle_start..cycle_start+length_of_cycle][remainder-1];

    println!("Part two: {}", part_two);
    Ok(())
}

// a single spot in the grid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Record {
    // A round rock
    Round,
    // A cubic rock
    Cubic,
    // An empty spot
    Empty,
}

impl Record {
    fn from_char(v: char) -> Self {
        match v {
            'O' => Record::Round,
            '#' => Record::Cubic,
            _ => Record::Empty,
        }
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Record::Round => 'O',
                Record::Cubic => '#',
                Record::Empty => '.',
            }
        )
    }
}

#[derive(Debug, Clone)]
struct Platform {
    grid: Vec<Vec<Record>>,
    rows: usize,
    cols: usize,
}

impl Platform {
    fn parse_platform(input: &str) -> Self {
        let grid: Vec<Vec<_>> = input
            .lines()
            .map(|l| l.chars().map(Record::from_char).collect())
            .collect();
        let cols = grid.iter().next().unwrap().len();
        let rows = grid.len();
        Self { grid, rows, cols }
    }

    fn swap(&mut self, (y_a, x_a): (usize, usize), (y_b, x_b): (usize, usize)) {
        let temp = self.grid[y_a][x_a];
        self.grid[y_a][x_a] = self.grid[y_b][x_b];
        self.grid[y_b][x_b] = temp;
    }

    fn tilt_north(&mut self) {
        for y in 0..self.cols {
            for x in 0..self.rows {
                if self.grid[y][x] != Record::Round {
                    continue;
                }

                // for every round rock, move it up as much as possible
                // by swapping with an empty spot above it
                let mut y_prev = y;
                for y_new in (0..y).rev() {
                    if self.grid[y_new][x] != Record::Empty {
                        break;
                    }
                    self.swap((y_new, x), (y_prev, x));
                    y_prev = y_new;
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for y in (0..self.cols).rev() {
            for x in 0..self.rows {
                if self.grid[y][x] != Record::Round {
                    continue;
                }

                let mut y_prev = y;
                for y_new in y + 1..self.cols {
                    if self.grid[y_new][x] != Record::Empty {
                        break;
                    }
                    self.swap((y_new, x), (y_prev, x));
                    y_prev = y_new;
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for x in 0..self.rows {
            for y in 0..self.cols {
                if self.grid[y][x] != Record::Round {
                    continue;
                }

                let mut x_prev = x;
                for x_new in (0..x).rev() {
                    if self.grid[y][x_new] != Record::Empty {
                        break;
                    }
                    self.swap((y, x_new), (y, x_prev));
                    x_prev = x_new;
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for x in (0..self.rows).rev() {
            for y in 0..self.cols {
                if self.grid[y][x] != Record::Round {
                    continue;
                }

                let mut x_prev = x;
                for x_new in x + 1..self.rows {
                    if self.grid[y][x_new] != Record::Empty {
                        break;
                    }
                    self.swap((y, x_new), (y, x_prev));
                    x_prev = x_new;
                }
            }
        }
    }

    fn compute_total_load(&self) -> usize {
        let mut load = 0;

        for y in 0..self.cols {
            let load_factor = self.cols - y;
            for x in 0..self.rows {
                if self.grid[y][x] != Record::Round {
                    continue;
                }
                load += load_factor;
            }
        }
        load
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for y in 0..self.cols {
            for x in 0..self.rows {
                write!(f, "{}", self.grid[y][x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


// https://en.wikipedia.org/wiki/Cycle_detection
fn lam<T: PartialEq+Debug>(v: &[T]) -> Option<usize> {
    let mut power = 1;
    let mut lam = 1;
    let mut tortoise = v.get(0)?;
    let mut hare_index = 1;
    let mut hare = v.get(hare_index)?;

    while tortoise != hare || lam <=2 {
        if power == lam {
            tortoise = hare;
            power *= 2;
            lam = 0;
        }
        hare_index += 1;
        hare = v.get(hare_index)?;
        lam += 1;
    }
    Some(lam)
}

// https://en.wikipedia.org/wiki/Cycle_detection
fn detect_cycle<T: PartialEq+Debug>(v: &[T]) -> Option<(usize,usize)> {
    let lam = lam(v)?;

    let mut hare_index = 0;
    let mut hare = v.get(hare_index)?;

    for _ in 0..lam {
        hare_index += 1;
        hare = v.get(hare_index)?;
    }

    let mut mu = 0;
    let mut tortoise_index = 0;
    let mut tortoise = v.get(tortoise_index)?;

    while tortoise != hare {
        tortoise_index += 1;
        tortoise = v.get(tortoise_index)?;

        hare_index += 1;
        hare = v.get(hare_index)?;

        mu += 1;
    }
    Some((mu,lam))
}
