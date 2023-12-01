mod part1;
mod part2;

use std::fs;

use anyhow::{Context, Result};
use clap::Parser;
use log::info;

#[derive(Parser, Debug)]
pub struct Command {}

impl common::CommandRunner for Command {
    fn run(&self) -> Result<()> {
        info!("Puzzle 1");

        let input_file = "data/puzzle_1/input.txt";

        let input = fs::read_to_string(input_file)
            .with_context(|| format!("Failed to read input file {input_file}"))?;

        let part1_sum = part1::sum_calibration_values(input.as_str())?;

        println!("Part 1: {part1_sum}");

        let part2_sum = part2::sum_calibration_values(input.as_str())?;

        println!("Part 2: {part2_sum}");

        Ok(())
    }
}
