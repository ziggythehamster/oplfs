use clap::{Parser, Subcommand};

use std::env;
use std::error::Error;
use std::path::PathBuf;

use crate::database;

/// String printed after help
const AFTER_HELP: &'static str = "For more help and setup examples, please see the README. This is open source software, see LICENSE for more information.";

/// oplfs global CLI options
#[derive(Parser, Debug)]
#[clap(version, about, after_help = AFTER_HELP, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// Database file to use, defaults to `oplfs.sqlite` in the current directory
    #[clap(global = true, short, long, value_parser, value_name = "FILE")]
    database: Option<PathBuf>,

    #[clap(subcommand)]
    command: Commands
}

/// Subcommands for oplfs
#[derive(Debug, Subcommand)]
enum Commands {
    /// Adds discs to the index
    Add {
        /// The path to crawl recursively
        #[clap(value_parser)]
        path: PathBuf
    }
}

/// Runs the CLI
pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // get the database path specified or the default one
    let database_path = match cli.database {
        Some(path) => path,
        None => {
            let mut path = env::current_dir()?;
            path.push("oplfs.sqlite");

            path
        }
    };

    let db_connection = database::open(database_path)?;

    // Deal with subcommands
    match &cli.command {
        Commands::Add { path } => {
            println!("Crawling {}", path.to_string_lossy());
        }
    }

    Ok(())
}
