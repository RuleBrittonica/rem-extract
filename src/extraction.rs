extern crate syn;
extern crate quote;

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

// ========================================
// Checks for the validity of the input
// ========================================

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

// ========================================
// Performs the method extraction
// ========================================
pub fn extract_method(input: ExtractionInput) -> Result<String, ExtractionError> {
    verify_input(&input)?;

    // Read the source code from the file
    let source_code: String = fs::read_to_string(&input.file_path).map_err(ExtractionError::Io)?;

    // Parse the source code into a syntax tree
    let syntax_tree: syn::File = parse_file(&source_code).map_err(ExtractionError::Parse)?;

    let start_cursor: Cursor = input.start_cursor;
    let end_cursor: Cursor = input.end_cursor;
    let start_line: usize = start_cursor.line;
    let start_column: usize = start_cursor.column;
    let end_line: usize = end_cursor.line;
    let end_column: usize = end_cursor.column;

    // For now, just write the source_code to the output file
    let output_code = &source_code;

    // Wrote the formatted code to the output file
    fs::write(&input.output_path, &output_code).map_err(ExtractionError::Io)?;

    // Call rustfmt to format the output file
    fmt_file(&input.output_path, &vec![]);

    Ok(output_code.to_string())

}

// ========================================
// Helper functions for extraction
// ========================================
