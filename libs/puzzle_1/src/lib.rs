use std::fs;

use log::{info, debug};
use clap::Parser;
use anyhow::{Result, bail, Context};

#[derive(Parser, Debug)]
pub struct Command {
}

impl common::CommandRunner for Command {
    fn run(&self) -> Result<()> {
        info!("Puzzle 1");

        let input_file = "data/puzzle_1/input.txt";

        let input = fs::read_to_string(input_file)
            .with_context(|| format!("Failed to read input file {input_file}"))?;

        debug!("{input}");
        
        bail!("TODO");
    }
}