mod commands;

use anyhow::Result;
use clap::Parser;
use commands::{ExecutableCommand, ProcessCommandRoot};

fn main() -> Result<()> {
    ProcessCommandRoot::parse().run()
}
