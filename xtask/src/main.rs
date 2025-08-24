//! summary: CLIエントリーポイント
//! path: xtask/src/main.rs

mod cli;
mod config;
mod render;
mod run;
mod summary;
mod tree;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Command::Layout {
            root,
            exclude,
            max_depth,
            truncate,
            respect_gitignore,
        } => run::run_layout(root, exclude, max_depth, truncate, respect_gitignore)?,
    }
    Ok(())
}
