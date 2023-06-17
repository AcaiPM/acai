#[path = "src/cli.rs"]
mod cli;

use std::{env, io};

use clap_complete::shells::{Bash, Elvish, Fish, PowerShell, Zsh};
use clap_complete_fig::Fig;

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");
    generate_completions()
}

fn generate_completions() -> io::Result<()> {
    const BIN_NAME: &str = env!("CARGO_PKG_NAME");
    const OUT_DIR: &str = "completions";
    let cli = &mut cli::cli();

    clap_complete::generate_to(Bash, cli, BIN_NAME, OUT_DIR)?;
    clap_complete::generate_to(Elvish, cli, BIN_NAME, OUT_DIR)?;
    clap_complete::generate_to(Fig, cli, BIN_NAME, OUT_DIR)?;
    clap_complete::generate_to(Fish, cli, BIN_NAME, OUT_DIR)?;
    clap_complete::generate_to(PowerShell, cli, BIN_NAME, OUT_DIR)?;
    clap_complete::generate_to(Zsh, cli, BIN_NAME, OUT_DIR)?;

    Ok(())
}
