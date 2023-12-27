use std::env;

use anyhow::{Context, Result};
use aoc_input_lib::get_puzzle_input;
use clap::Parser;

/// Lens Library
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
        get_puzzle_input(2023, 15, session).context("Could not retrieve puzzle input")?
    };

    let without_newlines = input.chars().filter(|ch| *ch != '\n').collect::<String>();

    let sum_part_one = without_newlines
        .split(',')
        .map(holiday_ascii_string_helper_algorithm)
        .sum::<u32>();

    println!("Part one: {}", sum_part_one);

    let mut boxes = vec![Box::new(); 256];

    for seq in without_newlines.split(',') {
        // instruction has = sign
        if seq.contains('=') {
            let mut it = seq.split('=');
            let label = it.next().unwrap();
            let focal_length = u32::from_str_radix(it.next().unwrap(), 10).unwrap();
            let lens = Lens {
                label: label.into(),
                focal_length,
            };

            let b = &mut boxes[holiday_ascii_string_helper_algorithm(label) as usize];

            b.lenses.push(lens);

            if let Some(other) = b
                .lenses
                .iter()
                .take(b.lenses.len() - 1)
                .position(|other| &other.label == label)
            {
                b.lenses.swap_remove(other);
            }
        }

        // instruction has - sign
        if seq.contains('-') {
            let label = seq.split('-').next().unwrap();
            let b = &mut boxes[holiday_ascii_string_helper_algorithm(label) as usize];
            if let Some(p) = b.lenses.iter().position(|other| &other.label == label) {
                b.lenses.remove(p);
            }
        }
    }

    let mut sum_part_two = 0;
    let mut box_power = 1;
    for b in boxes {
        let mut slot_power = 1;
        for lens in b.lenses {
            sum_part_two += box_power * slot_power * lens.focal_length;
            slot_power += 1;
        }
        box_power += 1;
    }
    println!("Part two: {}", sum_part_two);
    Ok(())
}

fn holiday_ascii_string_helper_algorithm(v: &str) -> u32 {
    let mut result = 0;
    for ch in v.chars() {
        let code_point = ch as u32;
        result = ((result + code_point) * 17) % 256
    }
    result
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u32,
}

#[derive(Debug, Clone)]
struct Box {
    lenses: Vec<Lens>,
}

impl Box {
    fn new() -> Self {
        Self { lenses: Vec::new() }
    }
}
