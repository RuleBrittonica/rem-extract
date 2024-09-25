use quote::{
    format_ident,
    quote
};
use rem_utils::fmt_file;
use std::{
    fs::{self, File},
    io::{
        self,
        BufReader,
        ErrorKind, Read
    },
};
use syn::{
    parse_file,
    ItemFn,
    Block,
    Ident,
    Expr,
};

use crate::{
    error::ExtractionError,
    rust_analyzer_local::size::TextSize,
    rust_analyzer_local::range::TextRange,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Cursor {
    pub line: usize, // Line in file, 1-indexed
    pub column: usize, // Column in line, 1-indexed
}

impl Cursor {
    pub fn new(line: usize, column: usize) -> Cursor {
        Cursor { line, column }
    }
}

#[derive(Debug, PartialEq, Clone)]
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
    // Since the cursor is 1-indexed, we need to check if the line number is 0
    if input.start_cursor.line == 0 {
        return Err(ExtractionError::ZeroLineIndex);
    }
    // Same for the end cursor
    if input.end_cursor.line == 0 {
        return Err(ExtractionError::ZeroLineIndex);
    }

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

// Function to extract the code segment based on cursor positions
pub fn extract_method(input: ExtractionInput) -> Result<String, ExtractionError> {

    // Get the cursor positions
    let start_cursor: Cursor = input.clone().start_cursor;
    let end_cursor: Cursor = input.clone().end_cursor;
    let start_line: usize = start_cursor.line;
    let start_column: usize = start_cursor.column;
    let end_line: usize = end_cursor.line;
    let end_column: usize = end_cursor.column;

    // Get info about the files
    let input_path: &str = &input.file_path;
    let output_path: &str = &input.output_path;
    let new_fn_name: &str = &input.new_fn_name;

    if (start_line == end_line) && (start_column == end_column) {
        return Err(ExtractionError::InvalidCursor);
    }

    verify_input(&input)?;

    // Read the source code from the file
    let source_code: String = fs::read_to_string(input_path).map_err(ExtractionError::Io)?;

    // Get the range of the code (start_line, start_column, end_line,
    // end_column, minus any leading / trailing whitespace)
    // At this stage, we reach out to rust-analyzer to get the AssistContext -
    // from there we can get everything else we need!



    Ok(source_code)

}

// ========================================
// Helper functions for extraction
// ========================================


// ========================================
// Utility functions
// ========================================