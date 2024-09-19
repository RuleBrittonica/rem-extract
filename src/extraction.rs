use quote::{
    format_ident,
    quote
};
use rem_utils;
use std::fs;
use syn::{
    parse_file,
    Item,
    Stmt
};

use crate::error::ExtractionError;

#[derive(Debug)]
pub struct ExtractionInput {
    pub file_path: String,
    pub output_file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub new_fn_name: String,
}

pub fn extract_method(input: ExtractionInput) -> Result<String, ExtractionError> {
    // Read the file and parse it into an AST
    let source_code = fs::read_to_string(&input.file_path)?;
    let parsed_file = parse_file(&source_code)?;

    // Extract the statements in the specified line range
    let mut extracted_stmts: Vec<Stmt> = vec![];
    for item in parsed_file.items {
        if let Item::Fn(ref func) = item {
            for (i, stmt) in func.block.stmts.iter().enumerate() {
                if i >= input.start_line && i <= input.end_line {
                    extracted_stmts.push(stmt.clone());
                }
            }
        }
    }

    if extracted_stmts.is_empty() {
        return Err(ExtractionError::InvalidLineRange);
    }

    // Create a valid identifier for the new function name
    let new_fn_ident = format_ident!("{}", input.new_fn_name);

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
            if i == input.start_line {
                format!("{}();", input.new_fn_name) // Insert a call to the new function
            } else if i > input.start_line && i <= input.end_line {
                String::new() // Remove the lines that were extracted
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    // Add the new function at the end of the file
    let output_code = format!("{}\n\n{}", modified_code, new_function);

    // Write the modified code to the original file
    fs::write(&input.file_path, &output_code)?;

    // Call rustfmt on the modified file
    let format_output = rem_utils::fmt_file(&input.file_path, &vec![]).output();
    if format_output.is_err() {
        return Err(ExtractionError::FormatError);
    }

    Ok(output_code)
}