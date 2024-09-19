use clap::{
    Parser,
    Subcommand,
    ArgAction,
};

use std::path::PathBuf;

use crate::messages::{
    version::VERSION,
    about::ABOUT,
    author::AUTHOR,
};

#[derive(Parser, Debug)]
#[command(
    version = VERSION,
    about = ABOUT,
    author = AUTHOR,
)]

pub struct Args {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Subcommand, Debug)]
pub enum Subcommand {
    Run {
        #[arg(help = "The path to the file to refactor", index = 1)]
        file_path: PathBuf,

        #[arg(help = "The output path for the refactored file", index = 2)]
        new_file_path: PathBuf,

        #[arg(help = "The start line of the code to extract", index = 3)]
        start_line: usize,

        #[arg(help = "The end line of the code to extract", index = 4)]
        end_line: usize,

        #[arg(help = "The name of the new function to create", index = 5)]
        new_fn_name: String,

        #[arg(short, long, help = "Enable verbose output", action = ArgAction::SetTrue)]
        verbose: bool,
    },

    Test {
        #[arg(short, long, help = "Enable verbose output", action = ArgAction::SetTrue)]
        verbose: bool,
    },
}