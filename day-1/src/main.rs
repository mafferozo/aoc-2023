use anyhow::{Context, Result};
use aoc_input_lib::get_puzzle_input;
fn main() -> Result<()> {
    println!("Hello, world!");
    let input = get_puzzle_input(2023, 1, None).context("Could not retrieve puzzle input")?;
    println!("{}", input);
    Ok(())
}
