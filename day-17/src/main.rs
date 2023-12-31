use std::{collections::HashMap, env};

use anyhow::{Context, Result};
use aoc_input_lib::get_puzzle_input;
use clap::Parser;

/// The Floor Will Be Lava
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
        get_puzzle_input(2023, 17, session).context("Could not retrieve puzzle input")?
    };
    println!("{input}");

    let g = Grid::parse_grid(&input);

    let mut stack = vec![(0, 0, Direction::Up, 0, Vec::default())];
    let mut visited: HashMap<(usize, usize), u32> = HashMap::new();

    while let Some((x, y, from_dir, cost, mut past_dirs)) = stack.pop() {
        // todo: check past_dirs
        if past_dirs.len() > 2 {
            let v = &past_dirs.as_slice()[past_dirs.len() - 3..];
        }
        let saved_cost = visited.entry((x, y)).or_insert(u32::MAX);
        if *saved_cost <= cost {
            continue;
        }
        *saved_cost = cost;

        for to_dir in from_dir.possible_dirs() {
            let Some((x, y)) = g.clamp((x, y), to_dir) else {
                continue;
            };
            let cost = cost + g.data[y][x];
            let mut pd = past_dirs.clone();
            pd.push(to_dir);
            stack.push((x, y, to_dir, cost, pd));
        }
    }
    assert!(visited.len() == g.rows * g.cols);
    dbg!(visited.get(&(g.cols - 1, g.rows - 1)));

    Ok(())
}

#[derive(Debug, Clone)]
struct Grid {
    data: Vec<Vec<u32>>,
    cols: usize,
    rows: usize,
}

impl Grid {
    fn parse_grid(input: &str) -> Self {
        let data = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|ch| (ch as u32) - ('0' as u32))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let cols = data.len();
        let rows = data.iter().next().unwrap().len();

        // sanity check: all values are parsed correctly
        assert!(data.iter().flat_map(|row| row.iter()).all(|x| *x < 10));
        Self { data, cols, rows }
    }

    fn clamp(&self, (x, y): (usize, usize), dir: Direction) -> Option<(usize, usize)> {
        use Direction::*;
        assert!(x < self.cols);
        assert!(y < self.rows);

        match dir {
            Up => (y > 0).then(|| (x, y - 1)),
            Right => (x + 1 < self.cols).then(|| (x + 1, y)),
            Left => (x > 0).then(|| (x - 1, y)),
            Down => (y + 1 < self.rows).then(|| (x, y + 1)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn possible_dirs(&self) -> [Self; 3] {
        use Direction::*;
        match self {
            Up => [Up, Left, Right],
            Down => [Down, Left, Right],
            Left => [Up, Down, Left],
            Right => [Up, Down, Right],
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
/// This struct is used for two things;
/// - Keeping track of the last three directions of a node, if there are any
/// 
/// - Computing the up to three edges of a node
struct Dirs([Option<Direction>; 3]);

impl Dirs {
    fn new() -> Self {
        Self([None, None, None])
    }

    fn append(&self, dir: Direction) -> Self {
        let new = Some(dir);

        // rotate past dirs
        Self([new, self.0[0], self.0[1]])
    }

    fn last(&self) -> Option<Direction> {
        self.0[0]
    }

    fn is_same(&self) -> bool {
        match (self.0[0], self.0[1], self.0[2]) {
            (Some(v1), Some(v2), Some(v3)) => v1==v2 && v1==v3,
            _ => false
        }
    }

    fn possible_dirs(&self) -> Self {
        use Direction::*;
        match self.0[0] {
            Some(v) => todo!(),
            None => Self([Some(Down), Some(Right), None]),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct Node {
    past_dirs: Dirs,
    pos: (usize,usize)
}

impl Node {

}
