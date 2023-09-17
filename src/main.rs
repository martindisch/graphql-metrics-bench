use std::path::PathBuf;

use clap::{Parser as ClapParser, Subcommand};
use eyre::Result;

mod commands;

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Stats { schema } => {
            commands::stats(&schema)?;
        }
    }

    Ok(())
}

#[derive(ClapParser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Prints stats about the given schema.
    Stats {
        /// Path to the schema file.
        schema: PathBuf,
    },
}
