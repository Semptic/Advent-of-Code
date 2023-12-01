use log::info;
use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
pub struct Command {
}

impl common::CommandRunner for Command  {
    fn run(&self) -> Result<()> {
        hello_world();

        Ok(())
    }
}

pub fn hello_world() {
    info!("hello world!");
}