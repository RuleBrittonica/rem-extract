mod logging;
mod messages;

mod extraction;
use extraction::{
    extract_method,
    ExtractionInput
};

mod rust_analyzer_local;

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
            start_line,
            start_column,
            end_line,
            end_column,
            new_fn_name,
            verbose,
        } => {
            info!("Running 'run' subcommand");
            info!("File Path: {:?}", file_path);
            info!("New File Path: {:?}", new_file_path);
            info!("Start Line: {}", start_line);
            info!("End Line: {}", end_line);
            info!("New Function Name: {}", new_fn_name);
            info!("Verbose: {}", if *verbose { "yes" } else { "no" });

            let input = ExtractionInput::new(
                file_path.to_str().unwrap(),
                new_file_path.to_str().unwrap(),
                new_fn_name,
                extraction::Cursor::new(*start_line, *start_column),
                extraction::Cursor::new(*end_line, *end_column),
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

