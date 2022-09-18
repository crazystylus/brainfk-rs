use brainfk_rs::cmd::{Cli, Run};
use clap::Parser;
use std::io;

fn main() -> Result<(), io::Error> {
    Cli::parse().run()
}
