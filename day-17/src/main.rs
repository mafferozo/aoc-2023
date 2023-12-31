use std::{collections::{HashMap, HashSet}, env};

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

    let mut stack = vec![(0, 0, Dirs::new(), 0)];
    // let mut visited: HashMap<(usize, usize), u32> = HashMap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some((x, y, past_dirs, cost)) = stack.pop() {
        // todo: check past_dirs
        // let saved_cost = visited.entry((x, y)).or_insert(u32::MAX);
        // if *saved_cost <= cost {
        //     continue;
        // }
        // *saved_cost = cost;
        if !visited.insert((x,y)) {
            continue
        }
        for to_dir in Dirs::possible_dirs(&g, past_dirs, (x, y)) {
            let (x,y) = to_dir.get_neighbour_unchecked((x,y));
            let cost = cost + g.data[y][x];
            stack.push((x, y, past_dirs.append(*to_dir), cost));
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

    fn possible_dirs(&self, pos: (usize, usize)) -> impl Iterator<Item = &Direction> {
        ALL_DIRS.iter().filter(move |d| self.is_in_grid(pos, d))
    }

    fn is_in_grid(&self, (x, y): (usize, usize), dir: &Direction) -> bool {
        use Direction::*;
        match dir {
            Up => y > 0,
            Down => y + 1 < self.rows,
            Left => x > 0,
            Right => x + 1 < self.cols,
        }
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

const ALL_DIRS: &[Direction; 4] = &[
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

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

    fn opposite_dir(&self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }

    fn get_neighbour_unchecked(&self, (x, y): (usize, usize)) -> (usize,usize) {
        use Direction::*;
        match self {
            Up => (x, y - 1),
            Down => (x, y + 1),
            Left => (x - 1, y),
            Right => (x + 1, y),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
/// Keep track of the last three directions of a path, if there are any
struct Dirs([Option<Direction>; 3]);

impl Dirs {
    fn new() -> Self {
        Self([None, None, None])
    }

    // Append a new direction to the log
    fn append(&self, dir: Direction) -> Self {
        let new = Some(dir);

        // rotate past dirs
        Self([new, self.0[0], self.0[1]])
    }

    // Returns the last direction
    fn last(&self) -> Option<Direction> {
        self.0[0]
    }

    /// Returns true if the past three directions are equal
    fn is_same(&self) -> bool {
        match (self.0[0], self.0[1], self.0[2]) {
            (Some(v1), Some(v2), Some(v3)) => v1 == v2 && v1 == v3,
            _ => false,
        }
    }

    /// Returns the possible directions of a location in the grid, filtered by:
    /// - their opposite direction (we can not go back)
    /// - the forward direction, if we already went forward three times
    fn possible_dirs(
        grid: &Grid,
        past_dirs: Dirs,
        pos: (usize, usize),
    ) -> impl Iterator<Item = &Direction> {
        grid.possible_dirs(pos).filter(move |d| {
            let last_dir = past_dirs.0[0];
            last_dir.is_none()
                || last_dir.is_some_and(|last_dir| {
                    last_dir != d.opposite_dir() || (**d == last_dir && !past_dirs.is_same())
                })
        })
    }
}
