use quote::{
    format_ident,
    quote
};
use rem_utils::fmt_file;
use std::{
    fs,
    io::{self, ErrorKind},
};
use syn::{
    parse_file,
    Item,
    Stmt
};

use crate::error::ExtractionError;

#[derive(Debug, PartialEq)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
}

impl Cursor {
    pub fn new(line: usize, column: usize) -> Cursor {
        Cursor { line, column }
    }
}

#[derive(Debug)]
pub struct ExtractionInput {
    pub file_path: String,
    pub output_path: String,
    pub new_fn_name: String,
    pub start_cursor: Cursor,
    pub end_cursor: Cursor,
}

impl ExtractionInput {
    pub fn new(
        file_path: &str,
        output_path: &str,
        new_fn_name: &str,
        start_cursor: Cursor,
        end_cursor: Cursor,
    ) -> ExtractionInput {
        ExtractionInput {
            file_path: file_path.to_string(),
            output_path: output_path.to_string(),
            new_fn_name: new_fn_name.to_string(),
            start_cursor,
            end_cursor,
        }
    }

    pub fn new_raw(
        file_path: &str,
        output_path: &str,
        new_fn_name: &str,
        start_cursor: usize,
        start_column: usize,
        end_cursor: usize,
        end_column: usize,
    ) -> ExtractionInput {
        ExtractionInput {
            file_path: file_path.to_string(),
            output_path: output_path.to_string(),
            new_fn_name: new_fn_name.to_string(),
            start_cursor: Cursor::new(start_cursor, start_column),
            end_cursor: Cursor::new(end_cursor, end_column),
        }
    }
}

// Check if the file exists and is readable
fn check_file_exists(file_path: &str) -> Result<(), ExtractionError> {
    if fs::metadata(file_path).is_err() {
        return Err(ExtractionError::Io(io::Error::new(
            ErrorKind::NotFound,
            format!("File not found: {}", file_path),
        )));
    }
    Ok(())
}

fn check_line_numbers(input: &ExtractionInput) -> Result<(), ExtractionError> {
    if input.start_cursor.line > input.end_cursor.line {
        return Err(ExtractionError::InvalidLineRange);
    }

    let source_code: String = fs::read_to_string(&input.file_path)?;
    let num_lines = source_code.lines().count();
    if input.end_cursor.line >= num_lines {
        return Err(ExtractionError::InvalidLineRange);
    }

    Ok(())
}

fn check_columns(input: &ExtractionInput) -> Result<(), ExtractionError> {
    if input.start_cursor.line == input.end_cursor.line
        && input.start_cursor.column > input.end_cursor.column
    {
        return Err(ExtractionError::InvalidColumnRange);
    }

    Ok(())
}

fn verify_input(input: &ExtractionInput) -> Result<(), ExtractionError> {
    // Execute each input validation step one by one
    check_file_exists(&input.file_path)?;
    check_line_numbers(input)?;
    check_columns(input)?;

    Ok(())
}
pub fn extract_method(input: ExtractionInput) -> Result<String, ExtractionError> {
    verify_input(&input)?;

    // Read the file and parse it into an AST
    let source_code: String = fs::read_to_string(&input.file_path)?;
    let parsed_file = parse_file(&source_code)?;

    // Extract the statements in the specified line range
    let mut extracted_stmts: Vec<Stmt> = vec![];
    for item in parsed_file.items {
        if let Item::Fn(ref func) = item {
            for (i, stmt) in func.block.stmts.iter().enumerate() {
                if i >= input.start_cursor.line && i <= input.end_cursor.line {
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
    let modified_code: String = source_code
        .lines()
        .enumerate()
        .map(|(i, line)| {
            if i == input.start_cursor.line {
                format!("{}();", input.new_fn_name) // Insert a call to the new function
            } else if i > input.start_cursor.line && i <= input.end_cursor.line {
                String::new() // Remove the lines that were extracted
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    // Add the new function at the end of the file
    let output_code: String = format!("{}\n\n{}", modified_code, new_function);

    // Write the modified code to the output path
    fs::write(&input.output_path, &output_code)?;

    // Call rustfmt on the modified file
    let format_output = fmt_file(&input.output_path, &vec![]).output();
    if format_output.is_err() {
        return Err(ExtractionError::FormatError);
    }

    Ok(output_code)
}