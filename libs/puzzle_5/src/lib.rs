mod part1;
mod part2;
mod utils;

use std::fs;

use anyhow::{bail, Context, Result};
use clap::Parser;
use log::info;

#[derive(Parser, Debug)]
pub struct Command {}

impl common::CommandRunner for Command {
    fn run(&self) -> Result<()> {
        info!("Puzzle 5");

        let input_file = "data/puzzle_5/input.txt";

        let input = fs::read_to_string(input_file)
            .with_context(|| format!("Failed to read input file {input_file}"))?;

        let almanac = utils::parse_input(&input).context("Failed to parse almanac")?;

        let part1_results: u32 = bail!("Not implemented");
        println!("Part 1: {part1_results}");

        let part2_results: u32 = bail!("Not implemented");
        println!("Part 2: {part2_results}");

        Ok(())
    }
}
