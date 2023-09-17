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
        Commands::Sample { schema } => {
            commands::sample(&schema)?;
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
    /// Prints a sample selection of fields.
    Sample {
        /// Path to the schema file.
        schema: PathBuf,
    },
}
