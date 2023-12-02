mod part1;
mod part2;
mod utils;

use std::fs;

use anyhow::{Context, Result};
use clap::Parser;
use log::info;

use crate::part1::calculate_result;
use crate::part2::sum_power;
use crate::utils::{parse_line, Cubes, Game};

#[derive(Parser, Debug)]
pub struct Command {}

impl common::CommandRunner for Command {
    fn run(&self) -> Result<()> {
        info!("Puzzle 1");

        let input_file = "data/puzzle_2/input.txt";

        let input = fs::read_to_string(input_file)
            .with_context(|| format!("Failed to read input file {input_file}"))?;

        let games: Result<Vec<Game>> = input
            .lines()
            .map(|line| parse_line(line).with_context(|| format!("Failed to parse {line}")))
            .collect();

        let games = games.context("Failed to parse games")?;

        let max = Cubes {
            red: 12,
            green: 13,
            blue: 14,
        };

        let part1_results = calculate_result(&games, &max);
        println!("Part 1: {part1_results}");

        let part2_results = sum_power(&games);
        println!("Part 2: {part2_results}");

        Ok(())
    }
}
