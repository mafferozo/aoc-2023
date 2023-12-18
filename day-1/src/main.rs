use std::env;

use anyhow::{anyhow, Context, Result};
use aoc_input_lib::get_puzzle_input;
use regex::Regex;

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();

    let input = if args.len() > 1 {
        args.pop().unwrap()
    } else {
        get_puzzle_input(2023, 1, None).context("Could not retrieve puzzle input")?
    };

    let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();

    // greedy match any character that comes before it when searching for the right part of the number
    // otherwise, twone will match "two" instead of "one"
    let re_from_right = Regex::new(r".*(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let mut sum_part_one = 0;
    let mut sum_part_two = 0;

    for line in input.lines() {
        // part one
        let first_digit_from_left = line
            .chars()
            .find(|c| c.is_digit(10))
            .context("Expected atleast 1 digit input")?
            .to_digit(10)
            .unwrap();
        let first_digit_from_right = line
            .chars()
            .rev()
            .find(|c| c.is_digit(10))
            .context("Expected atleast 1 digit input")?
            .to_digit(10)
            .unwrap();

        sum_part_one += first_digit_from_left * 10 + first_digit_from_right;

        // part two
        let first_digit_from_left = re
            .find_iter(line)
            .next()
            .context("Expected line to have atleast 1 match")?
            .as_str();
        let first_digit_from_left = parse_digit_or_str(first_digit_from_left)?;

        let first_digit_from_right = re_from_right
            .captures_iter(line)
            .last()
            .context("Expected line to have atleast 1 match")?
            .get(1)
            .unwrap()
            .as_str();
        let first_digit_from_right = parse_digit_or_str(first_digit_from_right)?;
        sum_part_two += first_digit_from_left * 10 + first_digit_from_right
    }
    println!("part_one: {}", sum_part_one);
    println!("part_two: {}", sum_part_two);
    Ok(())
}

fn parse_digit_or_str(v: &str) -> Result<u32> {
    u32::from_str_radix(v, 10).or_else(|_e| match v {
        "one" => Ok(1),
        "two" => Ok(2),
        "three" => Ok(3),
        "four" => Ok(4),
        "five" => Ok(5),
        "six" => Ok(6),
        "seven" => Ok(7),
        "eight" => Ok(8),
        "nine" => Ok(9),
        _ => Err(anyhow!("Could not parse {} into a u32", v)),
    })
}
