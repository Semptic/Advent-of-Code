mod part1;
mod part2;
mod utils;

use std::fs;

use anyhow::{Context, Result};
use clap::Parser;
use log::info;

use crate::utils::*;

#[derive(Parser, Debug)]
pub struct Command {}

impl common::CommandRunner for Command {
    fn run(&self) -> Result<()> {
        info!("Puzzle 4");

        let input_file = "data/puzzle_4/input.txt";

        let input = fs::read_to_string(input_file)
            .with_context(|| format!("Failed to read input file {input_file}"))?;

        let cards: Result<Vec<Card>> = input
            .lines()
            .map(|line| parse_line(line).with_context(|| format!("Failed to parse {line}")))
            .collect();

        let cards = cards.context("Failed to parse input")?;

        let part1_results: u32 = cards.iter().map(part1::get_card_points).sum();
        println!("Part 1: {part1_results}");

        // let part2_results = ???;
        // println!("Part 2: {part2_results}");

        Ok(())
    }
}
