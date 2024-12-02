mod part1;
mod part2;
mod utils;

use std::fs;

use anyhow::{Context, Result};
use clap::Parser;
use log::info;

use crate::part1::PlantDetails;

#[derive(Parser, Debug)]
pub struct Command {}

impl common::CommandRunner for Command {
    fn run(&self) -> Result<()> {
        info!("Puzzle 5");

        let input_file = "data/puzzle_5/input.txt";

        let input = fs::read_to_string(input_file)
            .with_context(|| format!("Failed to read input file {input_file}"))?;

        let mut lines = input.lines();
        let seed_line = lines.next().context("Failed to read seed line")?;
        let almanac_lines: Vec<_> = lines.collect();

        let almanac = utils::parse_input(&almanac_lines).context("Failed to parse almanac")?;

        let part1_seeds = part1::extract_seeds(seed_line).context("Failed to extract seeds")?;
        let part1_seed_iterator = part1_seeds.into_iter();
        let part1_results: PlantDetails =
            part1::get_lowest_location(part1_seed_iterator, &almanac)?;
        let part1_location = part1_results.location;
        println!("Part 1: {part1_location}");

        let part2_seed_ranges =
            part2::extract_seed_ranges(seed_line).context("Failed to extract seed ranges")?;
        let part2_seed_iterator = part2_seed_ranges
            .into_iter()
            .flat_map(|range| range.into_iter());
        let part2_results = part1::get_lowest_location(part2_seed_iterator, &almanac)?;
        let part2_location = part2_results.location;
        println!("Part 2: {part2_location}");

        Ok(())
    }
}
