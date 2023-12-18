use anyhow::{anyhow, Context, Result};
use aoc_input_lib::get_puzzle_input;
use clap::Parser;
use regex::Regex;

/// Cube Conundrum
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

    let re = Regex::new(r"^Game (\d+): (.*)").unwrap();

    let input = if let Some(s) = args.input {
        s
    } else {
        get_puzzle_input(2023, 2, args.session).context("Could not retrieve puzzle input")?
    };

    let mut sum_part_one = 0;
    let mut sum_part_two = 0;

    for line in input.lines() {
        // extract id
        for (_full, [id, sets]) in re.captures_iter(line).map(|c| c.extract()) {
            let id = u32::from_str_radix(id, 10)?;

            let sets: Result<Vec<_>> = sets.split(';').map(parse_set_of_cubes).collect();
            let sets = sets?;

            // part one
            if sets.iter().all(|(r,g,b)| *r <= 12 && *g <= 13 && *b <= 14) {
                sum_part_one += id;
            }

            // part two
            let (mut red_max, mut green_max, mut blue_max) = (0, 0, 0);
            for (r,g,b) in sets {
                red_max = u32::max(red_max, r);
                green_max = u32::max(green_max, g);
                blue_max = u32::max(blue_max, b);
            }
            sum_part_two += red_max*green_max*blue_max;
        }
    }
    println!("part one: {}", sum_part_one);
    println!("part two: {}", sum_part_two);

    Ok(())
}

fn parse_set_of_cubes(v: &str) -> Result<(u32, u32, u32)> {
    let (mut red, mut green, mut blue) = (0, 0, 0);
    for pair in v.split(',') {
        let mut words = pair.split_ascii_whitespace();

        let number_of_cubes: u32 =
            u32::from_str_radix(words.next().unwrap(), 10).context("Should be a number")?;
        let color = words.next().unwrap();

        match color {
            "red" => {
                red += number_of_cubes;
                Ok(())
            }
            "green" => {
                green += number_of_cubes;
                Ok(())
            }
            "blue" => {
                blue += number_of_cubes;
                Ok(())
            }
            _ => Err(anyhow!(
                "Invalid puzzle input: for pair of number of cubes and color: {} ",
                v
            )),
        }?;
    }
    Ok((red, green, blue))
}
