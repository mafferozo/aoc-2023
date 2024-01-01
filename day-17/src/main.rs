use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    env,
};

use anyhow::{Context, Result};
use aoc_input_lib::get_puzzle_input;
use clap::Parser;

/// Clumsy Crucible 
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
    let part_two = dijkstra_part_two(&g);
    println!("part two: {}", part_two);

    Ok(())
}

fn dijkstra_part_one(grid: &Grid) -> u32 {
    // initial state
    let mut queue = BinaryHeap::from(vec![
        State::new(0, Position::new((0, 0), Direction::Down), 0),
        State::new(0, Position::new((0, 0), Direction::Right), 0),
    ]);

    // set of visited nodes
    let mut visited: HashMap<(Position, usize), u32> = HashMap::new();

    let mut dist = vec![u32::MAX; grid.cols * grid.rows];
    dist[0] = 0;

    while let Some(State {
        cost,
        position,
        steps,
    }) = queue.pop()
    {
        // if we've already visited this node, we visited this node with a better heat value.
        // So skip this one.
        if let Some(old_heat) = visited.insert((position, steps), cost) {
            // sanity check: this really is true
            assert!(old_heat <= cost);
            continue;
        }

        // we can always rotate and move clock-wise
        if let Some(left) = grid.move_along_dir(position.loc(), position.dir.rotate_clockwise()) {
            let next = State::new(cost + grid.get(left.loc()), left, 1);
            // if this next state is the best option
            let (x, y) = next.position.loc();
            if next.cost < dist[y * grid.rows + x] {
                dist[y * grid.rows + x] = next.cost;
            }
            queue.push(next);
        }

        // we can always rotate and move counter-clock-wise
        if let Some(right) =
            grid.move_along_dir(position.loc(), position.dir.rotate_counter_clockwise())
        {
            let next = State::new(cost + grid.get(right.loc()), right, 1);
            // if this next state is the best option
            let (x, y) = next.position.loc();
            if next.cost < dist[y * grid.rows + x] {
                dist[y * grid.rows + x] = next.cost;
            }
            queue.push(next);
        }

        // we've already moved three times in this direction;
        // skip moving forward
        if steps >= 3 {
            continue;
        }
        if let Some(forward) = grid.move_along_dir(position.loc(), position.dir) {
            let next = State::new(cost + grid.get(forward.loc()), forward, steps + 1);
            // if this next state is the best option
            let (x, y) = next.position.loc();
            if next.cost < dist[y * grid.rows + x] {
                dist[y * grid.rows + x] = next.cost;
            }
            queue.push(next);
        }
    }

    return dist[(grid.cols - 1) * grid.cols + grid.rows - 1];
}

fn dijkstra_part_two(grid: &Grid) -> u32 {
    // initial state
    let mut queue = BinaryHeap::from(vec![
        State::new(0, Position::new((0, 0), Direction::Down), 0),
        State::new(0, Position::new((0, 0), Direction::Right), 0),
    ]);

    // set of visited nodes
    let mut visited: HashMap<(Position, usize), u32> = HashMap::new();

    let mut dist = vec![u32::MAX; grid.cols * grid.rows];
    dist[0] = 0;

    while let Some(State {
        cost,
        position,
        steps,
    }) = queue.pop()
    {
        // if we've already visited this node, we visited this node with a better heat value.
        // So skip this one.
        if let Some(old_heat) = visited.insert((position, steps), cost) {
            // sanity check: this really is true
            assert!(old_heat <= cost);
            continue;
        }

        // we can only turn after 4 steps
        if let Some(left) = grid
            .move_along_dir(position.loc(), position.dir.rotate_clockwise())
            .filter(|_| steps >= 4)
        {
            let next = State::new(cost + grid.get(left.loc()), left, 1);
            // if this next state is the best option
            let (x, y) = next.position.loc();
            if next.cost < dist[y * grid.rows + x] {
                dist[y * grid.rows + x] = next.cost;
            }
            queue.push(next);
        }

        // we can always rotate and move counter-clock-wise
        if let Some(right) = grid
            .move_along_dir(position.loc(), position.dir.rotate_counter_clockwise())
            .filter(|_| steps >= 4)
        {
            let next = State::new(cost + grid.get(right.loc()), right, 1);
            // if this next state is the best option
            let (x, y) = next.position.loc();
            if next.cost < dist[y * grid.rows + x] {
                dist[y * grid.rows + x] = next.cost;
            }
            queue.push(next);
        }

        // we've already moved three times in this direction;
        // skip moving forward
        if let Some(forward) = grid
            .move_along_dir(position.loc(), position.dir)
            .filter(|_| steps < 10)
        {
            let next = State::new(cost + grid.get(forward.loc()), forward, steps + 1);
            // if this next state is the best option
            let (x, y) = next.position.loc();
            if next.cost < dist[y * grid.rows + x] {
                dist[y * grid.rows + x] = next.cost;
            }
            queue.push(next);
        }
    }

    return dist[(grid.cols - 1) * grid.cols + grid.rows - 1];
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

        // number of cols, number of rows should fit in a isize
        Self { data, cols, rows }
    }

    /// Move the current position `(x,y)` along Direction `dir`
    /// Returns `Some(x_new,y_new)` if the move is valid; i.e. the new position lies in the grid.
    fn move_along_dir(&self, (x, y): (usize, usize), dir: Direction) -> Option<Position> {
        use Direction::*;
        assert!(x < self.cols);
        assert!(y < self.rows);

        let v = match dir {
            Up => (y > 0).then(|| (x, y - 1)),
            Right => (x + 1 < self.cols).then(|| (x + 1, y)),
            Left => (x > 0).then(|| (x - 1, y)),
            Down => (y + 1 < self.rows).then(|| (x, y + 1)),
        };
        v.map(|v| Position::new(v, dir))
    }

    fn get(&self, (x, y): (usize, usize)) -> u32 {
        self.data[y][x]
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
            Up => Right,
            Left => Up,
            Down => Left,
            Right => Down,
        }
    }

    fn rotate_counter_clockwise(&self) -> Self {
        use Direction::*;
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Position {
    fn new((x, y): (usize, usize), dir: Direction) -> Self {
        Self { x, y, dir }
    }

    fn loc(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

/// Represents a state of a single path the Crucible can walk
#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    /// The accumulated heat cost of the path
    cost: u32,
    /// The current position
    position: Position,
    /// The current amount of steps in the same direction
    steps: usize,
}

impl State {
    fn new(cost: u32, position: Position, steps: usize) -> Self {
        Self {
            cost,
            position,
            steps,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.steps.cmp(&other.steps))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
