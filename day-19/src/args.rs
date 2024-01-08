use clap::Parser;

/// Lavaduct Lagoon
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = Option::None)]
pub struct Args {
    /// The puzzle input
    #[arg()]
    pub input: Option<String>,

    /// Advent of code session token
    #[arg(short, long)]
    pub session: Option<String>,
}