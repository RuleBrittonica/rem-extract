mod error;
mod logging;
mod messages;

mod extraction;
use extraction::{
    ExtractionInput,
    extract_method
};



mod args;
use args::{};

fn main() {
    logging::init_logging();

    info!("Application Started");

    // Path to the Rust source file
    let file_path = "input/simple_example.rs";

    // Specify the line range to extract (0-based indexing)
    let start_line = 1;  // Corresponds to `let x = 10;`
    let end_line = 3;    // Corresponds to `let sum = x + y;`

    // Name of the new function to create
    let new_fn_name = "calculate_sum";

    // Prepare the extraction input
    let input = ExtractionInput {
        file_path: file_path.to_string(),
        start_line,
        end_line,
        new_fn_name: new_fn_name.to_string(),
    };

    // Extract the method and get the modified code
    let _ = extract_method(input);

    println!("Modified code has been written and formatted.");
}
