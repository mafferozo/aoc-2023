use std::{
    collections::HashMap,
    env,
    fmt::Display,
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
        get_puzzle_input(2023, 16, session).context("Could not retrieve puzzle input")?
    };

    let g = Grid::parse_grid(&input);
    let mut positions = HashMap::new();
    mark_tiles(&g, Some((0, 0)), BeamDirection::East, &mut positions);

    // for y in 0..g.cols {
    //     for x in 0..g.rows {
    //         if positions.contains_key(&(x, y)) {
    //             print!("#");
    //         } else {
    //             print!("{}", g.tiles[y][x])
    //         }
    //     }
    //     println!();
    // }

    println!("Part one: {}", positions.keys().len());

    // part two:
    let mut inital_positions = vec![];
    for y in 0..g.rows {
        inital_positions.push((0,y,BeamDirection::East));
        inital_positions.push((g.cols-1,y,BeamDirection::West));
    }

    for x in 0..g.cols {
        inital_positions.push((x,0, BeamDirection::South));
        inital_positions.push((x,g.rows-1, BeamDirection::North))
    }

    let mut max = 0;
    for (x,y,beam_dir) in inital_positions {
        let mut positions = HashMap::new();
        mark_tiles(&g, Some((x,y)), beam_dir, &mut positions);
        max = max.max(positions.keys().len());
    }
    println!("Part two: {}", max);

    Ok(())
}

fn mark_tiles(
    grid: &Grid,
    pos: Option<(usize, usize)>,
    beam_dir: BeamDirection,
    positions: &mut HashMap<(usize, usize), Vec<BeamDirection>>,
) {
    // Invoked mark_tiles with a position outside the grid, simply return
    let Some(pos) = pos else {
        return;
    };

    // Direction for this tile/position that is already computed
    let seen_directions = positions.entry(pos).or_default();

    // we've already been on this tile, for this incoming beam direction
    if seen_directions.contains(&beam_dir) {
        return;
    } else {
        seen_directions.push(beam_dir);
    }

    // match on this tile
    match grid.tiles[pos.1][pos.0] {
        Tile::Empty => mark_tiles(grid, grid.clamp(pos, beam_dir), beam_dir, positions),
        Tile::LeftMirror => {
            let beam_dir = beam_dir.mirror_beam_left();
            mark_tiles(grid, grid.clamp(pos, beam_dir), beam_dir, positions);
        }
        Tile::RightMirror => {
            let beam_dir = beam_dir.mirror_beam_right();
            mark_tiles(grid, grid.clamp(pos, beam_dir), beam_dir, positions);
        }
        Tile::HorizontalSplitter => {
            for beam_dir in beam_dir.split_beam_horizontally() {
                mark_tiles(grid, grid.clamp(pos, beam_dir), beam_dir, positions);
            }
        }
        Tile::VerticalSplitter => {
            for beam_dir in beam_dir.split_beam_vertically() {
                mark_tiles(grid, grid.clamp(pos, beam_dir), beam_dir, positions);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    /// symbol: .
    Empty,
    /// symbol: /
    LeftMirror,
    /// symbol: \
    RightMirror,
    /// symbol: -
    HorizontalSplitter,
    /// symbol: |
    VerticalSplitter,
}

impl Tile {
    fn from_char(v: char) -> Self {
        use Tile::*;
        match v {
            '/' => LeftMirror,
            '\\' => RightMirror,
            '-' => HorizontalSplitter,
            '|' => VerticalSplitter,
            _ => Empty,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => '.',
                Tile::LeftMirror => '/',
                Tile::RightMirror => '\\',
                Tile::HorizontalSplitter => '-',
                Tile::VerticalSplitter => '|',
            }
        )
    }
}

#[derive(Debug, Clone)]
struct Grid {
    rows: usize,
    cols: usize,
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn parse_grid(input: &str) -> Self {
        let data = input
            .lines()
            .map(|l| l.chars().map(Tile::from_char).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let rows = data.len();
        let cols = data.iter().next().unwrap().len();
        Self {
            rows,
            cols,
            tiles: data,
        }
    }

    fn clamp(&self, (x, y): (usize, usize), beam_dir: BeamDirection) -> Option<(usize, usize)> {
        assert!(x < self.cols);
        assert!(y < self.rows);

        match beam_dir {
            BeamDirection::North => (y > 0).then(|| (x, y - 1)),
            BeamDirection::East => (x + 1 < self.cols).then(|| (x + 1, y)),
            BeamDirection::West => (x > 0).then(|| (x - 1, y)),
            BeamDirection::South => (y + 1 < self.rows).then(|| (x, y + 1)),
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for y in 0..self.cols {
            for x in 0..self.rows {
                write!(f, "{}", self.tiles[y][x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BeamDirection {
    North,
    East,
    West,
    South,
}

impl BeamDirection {
    /// symbol: /
    fn mirror_beam_left(self) -> BeamDirection {
        use BeamDirection::*;
        match self {
            North => East,
            East => North,
            West => South,
            South => West,
        }
    }

    /// symbol: \
    fn mirror_beam_right(self) -> BeamDirection {
        use BeamDirection::*;
        match self {
            North => West,
            East => South,
            West => North,
            South => East,
        }
    }

    //// symbol: -
    fn split_beam_horizontally(self) -> Vec<BeamDirection> {
        use BeamDirection::*;
        match self {
            North => vec![West, East],
            East => vec![self],
            West => vec![self],
            South => vec![West, East],
        }
    }

    //// symbol: |
    fn split_beam_vertically(self) -> Vec<BeamDirection> {
        use BeamDirection::*;
        match self {
            North => vec![self],
            East => vec![North, South],
            West => vec![North, South],
            South => vec![self],
        }
    }
}
