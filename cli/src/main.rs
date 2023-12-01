extern crate ansi_term;
extern crate clap_verbosity_flag;
extern crate loggerv;
use anyhow::{Context, Result};

use common::CommandRunner;
use log::debug;

use clap::{Parser, Subcommand};

fn main() -> Result<()> {
    let args = Args::parse();

    setup(&args).context("Failed to setup application environment")?;

    args.puzzle.run()?;

    Ok(())
}

fn setup(opt: &Args) -> Result<()> {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().context("Failed to enable ansi support")?;

    loggerv::Logger::new()
        .max_level(
            opt.verbosity
                .log_level()
                .context("Failed to get log level")?,
        )
        .level(opt.debug)
        .module_path(opt.debug)
        .line_numbers(opt.debug)
        .init()
        .context("Failed to setup logger")?;

    debug!("{:#?}", *opt);

    Ok(())
}

#[derive(Subcommand, Debug)]
enum Puzzles {
    HelloWorld(hello_world::Command),
    Puzzle1(puzzle_1::Command),
}

impl Puzzles {
    fn run(&self) -> Result<()> {
        match self {
            Puzzles::HelloWorld(cmd) => cmd.run().context("Failed to run hello world"),
            Puzzles::Puzzle1(cmd) => cmd.run().context("Failed to run puzzle 1"),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, name = "aoc", about = "My take on Advent of Code 2023")]
struct Args {
    #[structopt(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,

    /// Enables debug mode
    #[structopt(short, long)]
    debug: bool,

    #[command(subcommand)]
    puzzle: Puzzles,
}