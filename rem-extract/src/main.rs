mod logging;
mod messages;
mod extraction_utils;

mod extraction;
use extraction::{
    extract_method,
    ExtractionInput
};

use log::{
    // error,
    info
};

mod args;
use args::{
    EXTRACTArgs,
    EXTRACTCommands
};

mod extract_tests;
mod test_details;
use extract_tests::{
    test,
    test_verbose,
};

mod error;

use clap::Parser;

fn main() {
    logging::init_logging();

    info!("Application Started");

    let args: EXTRACTArgs = EXTRACTArgs::parse();

    match &args.command {
        EXTRACTCommands::Extract {
            file_path,
            new_file_path,
            new_fn_name,
            start_index,
            end_index,
            verbose,
        } => {
            info!("Running 'run' subcommand");
            info!("File Path: {:?}", file_path);
            info!("New Function Name: {}", new_fn_name);
            info!("Start Index: {}", start_index);
            info!("End Index: {}", end_index);
            info!("Verbose: {}", if *verbose { "yes" } else { "no" });

            let input = ExtractionInput::new(
                file_path.to_str().unwrap(),
                new_file_path.to_str().unwrap(),
                new_fn_name,
                *start_index as u32,
                *end_index as u32,
            );

            let _modified_code = extract_method(input);
        }

        EXTRACTCommands::Test { verbose } => {
            info!("Running 'test' subcommand");
            info!("Verbose: {}", if *verbose { "yes" } else { "no" });
            if *verbose {
                test_verbose();
            } else {
                test();
            }
        }

    }
}

