mod logging;
mod messages;

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

mod test;
use test::test;

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
            end_line,
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

            let input = ExtractionInput {
                file_path: file_path.to_string_lossy().to_string(),
                output_file_path: new_file_path.to_string_lossy().to_string(), // Updated
                start_line: *start_line,
                end_line: *end_line,
                new_fn_name: new_fn_name.to_string(),
            };

            let _modified_code = extract_method(input);
        }

        EXTRACTCommands::Test { verbose } => {
            info!("Running 'test' subcommand");
            info!("Verbose: {}", if *verbose { "yes" } else { "no" });
            test()
        }

    }
}

