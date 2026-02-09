use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "fhist",
    version,
    about = "File history tracker",
    long_about = "Track changes of files and view their history from the terminal.\n\
A minimal utility to subscribe to files and inspect how they evolve."
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Add a file to be tracked.
    ///
    /// Example:
    ///     fhist add notes.txt
    Add {
        /// Path to file
        target: String,
    },
    /// Stop tracking a file.
    ///
    /// You can pass either an ID from `fhist list`
    /// or the original file path.
    Remove {
        /// File id or path
        target: String,
    },
    /// List all tracked files with thier IDs and paths.
    List,

    /// Show the history of changes for a specific tracked file
    Log {
        /// File if or path
        target: String,
    },
}
