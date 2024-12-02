#![feature(vec_pop_if)]
mod part1;
mod part2;

use std::fs::File;

use anyhow::Result;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Command {}

impl common::CommandRunner for Command {
    fn run(&self) -> Result<()> {
        let input_file = "data/puzzle_1/input.txt";

        let file = File::open(input_file)?;

        let part1_result = part1::run(file)?;
        println!("Part 1: {part1_result}");

        let file = File::open(input_file)?;
        let part2_result = part2::run(file)?;
        println!("Part 2: {part2_result}");

        Ok(())
    }
}