use std::{collections::HashSet, env};

use anyhow::{Context, Result};
use aoc_input_lib::get_puzzle_input;
use clap::Parser;
use regex::Regex;

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
        get_puzzle_input(2023, 10, session).context("Could not retrieve puzzle input!")?
    };

    let mut grid = Grid::parse_input(&input);
    let start_pos: (i32, i32) = grid.find_char('S').unwrap();

    // smallest y, largest x
    let mut smallest = (0, i32::MAX);
    let mut is_clockwise = true;
    let mut positions = HashSet::new();
    let mut steps = 1;
    let first_pos = find_first_connected_pipe(start_pos, &grid);
    let mut cur = first_pos;
    let mut prev = start_pos;

    // walk algorithm
    loop {
        // get current pipe from the grid
        let current = *grid.get(cur.0, cur.1);
        positions.insert(cur);
        // we're done if we reached S again
        if current == 'S' {
            break;
        }
        // positions, smallest, steps:
        // gather some data during the walk
        let (x,y) = cur;
        let (x_prev,_y_prev) = prev;
        if y < smallest.1 {
            smallest = (x, y);
            is_clockwise = x != x_prev;
        } else if y == smallest.1 {
            if x > smallest.0 {
                smallest = (x, y);
                is_clockwise = x != x_prev;
            }
        }
        steps += 1;

        // find the next pipe that is part of the loop 
        (cur, prev) = (next(current, cur,prev), cur);
    }
    dbg!(positions.len());
    println!("Part one: {}", steps / 2);

    // remove all extranous pipes not part of the loop
    grid.iter_mut(|x,y, ch| {
        if !positions.contains(&(x,y)) {
            *ch = '.';
        }
    });
    let cross_once =[r"L-*7", r"F-*J"].map(|r| Regex::new(r).unwrap());
    let cross_twice =[r"L-*J", r"F-*7"].map(|r| Regex::new(r).unwrap());
    // count how many times we cross per line
    let mut sum_part_two = 0;
    for line in grid.data {
        let mut s = line.iter().collect::<String>();
        for re in cross_once.iter() {
            s = re.replace_all(&s, "|").into()
        }
        for re in cross_twice.iter() {
            s = re.replace_all(&s, "||").into()
        }

        dbg!(&s);
        let mut cross = 0;
        let mut inside = 0;
        for c in s.chars() {
            if c == '.' && cross % 2 != 0 {
                inside += 1;
            } else if  ['S','F','7','L','J','|'].contains(&c) {
                cross += 1;
            }
        }
        sum_part_two += inside;
    }
    println!("Part two: {}", sum_part_two);
    Ok(())
}

fn find_first_connected_pipe(s_position: (i32, i32), grid: &Grid) -> (i32, i32) {
    let (x,y) = s_position;
    // right
    if x + 1 < grid.cols() as i32 && ['7', 'J', '-'].contains(grid.get(x, y)) {
        return (x + 1, y);
    }
    // left
    if x - 1 > 0 && ['F', 'L', '-'].contains(grid.get(x - 1, y)) {
        return (x - 1, y);
    }
    // down
    if y+1 < grid.rows() as i32 && ['J', 'L', '|'].contains(grid.get(x, y + 1)) {
        return (x,y+1)
    } 
    // assume it is up
    (x,y-1)
}

fn next(current: char, cur: (i32, i32), prev: (i32, i32)) -> (i32, i32) {
    let (mut x, mut y) = cur;
    let (x_prev, y_prev) = prev;
    assert!(x != x_prev || y != y_prev);

    if x < x_prev {
        match current {
            '-' => x -= 1,
            'F' => y += 1,
            'L' => y -= 1,
            _ => panic!("wrong puzzle input"),
        }
    } else if x > x_prev {
        match current {
            '-' => x += 1,
            '7' => y += 1,
            'J' => y -= 1,
            _ => panic!("wrong puzzle input"),
        }
    } else if y < y_prev {
        match current {
            '|' => y -= 1,
            'F' => x += 1,
            '7' => x -= 1,
            _ => panic!("wrong puzzle input"),
        }
    } else if y > y_prev {
        match current {
            '|' => y += 1,
            'L' => x += 1,
            'J' => x -= 1,
            _ => panic!("wrong puzzle input"),
        }
    }
    (x, y)
}

struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    fn parse_input(input: &str) -> Grid {
        let data = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Grid { data }
    }

    fn find_char(&self, search: char) -> Option<(i32, i32)> {
        let mut position = None;
        for y in 0..self.rows() {
            for x in 0..self.cols() {
                let (x, y): (i32, i32) = (x as i32, y as i32);
                let ch = *self.get(x, y);
                if ch == search {
                    position = Some((x, y));
                    break;
                }
            }
        }
        position
    }

    fn iter_mut(&mut self, f: impl Fn(i32,i32, &mut char)) {
        for y in 0..self.rows() {
            for x in 0..self.cols() {
                let (x, y): (i32, i32) = (x as i32, y as i32);
                f(x,y, self.get_mut(x, y));
                // print!("{}", self.get(x,y))
            }
            // print!("{}", "\n")
        }
    }

    fn cols(&self) -> usize {
        self.data[0].len()
    }

    fn rows(&self) -> usize {
        self.data.len()
    }

    fn get(&self, x: i32, y: i32) -> &char {
        &self.data[y as usize][x as usize]
    }

    fn get_mut(&mut self, x: i32, y: i32) -> &mut char {
        &mut self.data[y as usize][x as usize]
    }
}
