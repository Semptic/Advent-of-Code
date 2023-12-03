mod part1;
mod part2;
mod utils;

use std::fs;

use anyhow::{Context, Result};
use clap::Parser;
use log::info;

use crate::utils::load_input;

#[derive(Parser, Debug)]
pub struct Command {}

impl common::CommandRunner for Command {
    fn run(&self) -> Result<()> {
        info!("Puzzle 3");

        let input_file = "data/puzzle_3/input.txt";

        let input = fs::read_to_string(input_file)
            .with_context(|| format!("Failed to read input file {input_file}"))?;

        let engine = load_input(input.as_str());

        let part1_result: u32 = part1::extract_part_numbers(&engine)
            .context("Failed to extract part 1")?
            .iter()
            .sum();

        println!("Part 1: {part1_result}");

        Ok(())
    }
}
