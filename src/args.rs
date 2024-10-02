use clap::{ArgAction, Parser, Subcommand};

use std::path::PathBuf;

use crate::messages::{about::ABOUT, author::AUTHOR, version::VERSION};

#[derive(Parser)]
#[command(
    version = VERSION,
    about = ABOUT,
    author = AUTHOR,
)]
pub struct EXTRACTArgs {
    #[command(subcommand)]
    pub command: EXTRACTCommands,
}

#[derive(Subcommand)]
pub enum EXTRACTCommands {
    // Run the extraction process with specific arguments
    Extract {
        #[arg(help = "The path to the file to refactor")]
        file_path: PathBuf,

        #[arg(help = "The output path for the refactored file")]
        new_file_path: PathBuf,

        #[arg(help = "The start line of the code to extract")]
        start_line: usize,

        #[arg(help = "The column of the cursor on the start line")]
        start_column: usize,

        #[arg(help = "The end line of the code to extract")]
        end_line: usize,

        #[arg(help = "The column of the cursor on the end line")]
        end_column: usize,

        #[arg(help = "The name of the new function to create")]
        new_fn_name: String,

        #[arg(short, long, help = "Enable verbose output", action = ArgAction::SetTrue)]
        verbose: bool,
    },

    // Test the extraction process
    Test {
        #[arg(short, long, help = "Enable verbose output", action = ArgAction::SetTrue)]
        verbose: bool,
    },
}
