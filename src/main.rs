mod logging;
mod error;
mod extraction;

use syn::{parse_file, Item, Stmt};
use quote::{quote, format_ident};
use std::fs;
use std::process::Command;

fn extract_method(file_path: &str, start_line: usize, end_line: usize, new_fn_name: &str) -> String {
    // Read the file and parse it into an AST
    let source_code = fs::read_to_string(file_path).expect("Failed to read file");
    let parsed_file = parse_file(&source_code).expect("Failed to parse Rust file");

    // Extract the statements in the specified line range
    let mut extracted_stmts: Vec<Stmt> = vec![];
    for item in parsed_file.items {
        if let Item::Fn(ref func) = item {
            for (i, stmt) in func.block.stmts.iter().enumerate() {
                if i >= start_line && i <= end_line {
                    extracted_stmts.push(stmt.clone());
                }
            }
        }
    }

    // Create a valid identifier for the new function name
    let new_fn_ident = format_ident!("{}", new_fn_name);

    // Create the new function using `quote!`
    let new_function = quote! {
        fn #new_fn_ident() {
            #(#extracted_stmts)*
        }
    };

    // Replace the original lines with a call to the new function
    let modified_code = source_code
        .lines()
        .enumerate()
        .map(|(i, line)| {
            if i == start_line {
                format!("{}();", new_fn_name) // Insert a call to the new function
            } else if i > start_line && i <= end_line {
                String::new() // Remove the lines that were extracted
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    // Add the new function at the end of the file
    format!("{}\n\n{}", modified_code, new_function)
}

fn main() {
    // Path to the Rust source file
    let file_path = "input/simple_example.rs";

    // Specify the line range to extract (0-based indexing)
    let start_line = 1;  // Corresponds to `let x = 10;`
    let end_line = 3;    // Corresponds to `let sum = x + y;`

    // Name of the new function to create
    let new_fn_name = "calculate_sum";

    // Extract the method and get the modified code
    let modified_code = extract_method(file_path, start_line, end_line, new_fn_name);

    // Specify the output file path
    let output_path = "output/simple_example.rs";

    // Write the modified code to the output file
    fs::write(output_path, modified_code).expect("Failed to write to output file");

    // Run `rustfmt` on the output file to format it
    let _ = Command::new("rustfmt")
        .arg(output_path)
        .output()
        .expect("Failed to format the output file with rustfmt");

    println!("Modified and formatted code has been written to {}", output_path);
}