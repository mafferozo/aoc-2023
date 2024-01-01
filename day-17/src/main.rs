use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    env,
};

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
    let g = Grid::parse_grid(&input);

    let part_one = dijkstra_part_one(&g);
    println!("part one: {}", part_one);

    Ok(())
}

fn dijkstra_part_one(g: &Grid) -> u32 {
    // initial state
    let mut queue = BinaryHeap::from(vec![
        State::new(0, Position::new((0, 0), Direction::Down), 1),
        State::new(0, Position::new((0, 0), Direction::Right), 1),
    ]);

    // set of visited nodes
    let mut visited: HashSet<Position> = HashSet::new();

    // target location
    let target = (g.cols - 1, g.rows - 1);

    let mut heat = 0;

    while let Some(state) = queue.pop() {
        let loc = (state.position.x, state.position.y);

        // if we arrived at the target location, return the answer
        if loc == target {
            heat = state.heat;
            break;
        }

        // if we've already visited this node, we visited this node with a better heat value.
        // So skip this one.
        if !visited.insert(state.position) {
            continue;
        }

        // we can always rotate and move clock-wise
        let dir = state.position.dir.rotate_clockwise();
        if let Some(new_loc) = g.move_along_dir(loc, &dir) {
            queue.push(State::new(
                state.heat + g.get(new_loc),
                Position::new(new_loc, dir),
                1,
            ))
        }

        // we can always rotate and move counter-clock-wise
        let dir = state.position.dir.rotate_counter_clockwise();
        if let Some(new_loc) = g.move_along_dir(loc, &dir) {
            queue.push(State::new(
                state.heat + g.get(new_loc),
                Position::new(new_loc, dir),
                1,
            ))
        }

        // we've already moved three times in this direction
        if state.steps > 3 {
            continue;
        }
        let dir = state.position.dir;
        if let Some(new_loc) = g.move_along_dir(loc, &dir) {
            queue.push(State::new(
                state.heat + g.get(new_loc),
                Position::new(new_loc, dir),
                state.steps + 1,
            ))
        }
    }
    return heat;
}

#[derive(Debug, Clone)]
struct Grid {
    data: Vec<Vec<u32>>,
    cols: isize,
    rows: isize,
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

        // number of cols, number of rows should fit in a isize
        Self {
            data,
            cols: isize::try_from(cols).unwrap(),
            rows: isize::try_from(rows).unwrap(),
        }
    }

    /// Move the current position `(x,y)` along Direction `dir`
    /// Returns `Some(x_new,y_new)` if the move is valid; i.e. the new position lies in the grid.
    fn move_along_dir(&self, (x, y): (isize, isize), dir: &Direction) -> Option<(isize, isize)> {
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

    fn get(&self, (x, y): (isize, isize)) -> u32 {
        self.data[y as usize][x as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate_clockwise(&self) -> Self {
        use Direction::*;
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    fn rotate_counter_clockwise(&self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Left => Up,
            Down => Left,
            Right => Down,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: isize,
    y: isize,
    dir: Direction,
}

impl Position {
    fn new((x, y): (isize, isize), dir: Direction) -> Self {
        Self { x, y, dir }
    }
}

/// Represents a state of a single path the Crucible can walk
#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    /// The accumulated heat of the path
    heat: u32,
    /// The current position
    position: Position,
    /// The current amount of steps in the same direction
    steps: usize,
}

impl State {
    fn new(heat: u32, position: Position, steps: usize) -> Self {
        Self {
            heat,
            position,
            steps,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .heat
            .cmp(&self.heat)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| other.steps.cmp(&self.steps))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
