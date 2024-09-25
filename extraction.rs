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
    rust_analyzer::size::TextSize,
    rust_analyzer::range::TextRange,
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
    // At this stage we convert to using rust-analyzers internal representation
    // of the code to make it easier to eventually transition to merging to
    // rust-analyzer
    let initial_range: TextRange = cursor_to_range(input_path, start_cursor, end_cursor)?;
    let range = trim_whitespace(&source_code, initial_range);


    // If the selected code is just a comment, return an error

    // If the

    Ok(source_code)

}

// ========================================
// Helper functions for extraction
// ========================================


// ========================================
// Utility functions
// ========================================

// Converts the cursor representation into a rust-analyzer TextRange
fn cursor_to_range(
    input_file_path: &str,
    start_cursor: Cursor,
    end_cursor: Cursor
) -> Result<TextRange, ExtractionError> {
    let start_offset: TextSize = cursor_to_size(input_file_path, start_cursor)?;
    let end_offset: TextSize = cursor_to_size(input_file_path, end_cursor)?;

    Ok(TextRange::new(start_offset, end_offset))
}

// Converts the cursor representation into a rust-analyzer TextSize
fn cursor_to_size(input_file_path: &str, cursor: Cursor) -> Result<TextSize, ExtractionError> {
    // Ensure cursor line is non-zero since cursor is 1-indexed
    if cursor.line == 0 {
        return Err(ExtractionError::ZeroLineIndex);
    }

    let lines: Vec<String> = read_lines(input_file_path);
    let mut count: u32 = 0;

    // Loop over every line, adding the length of the line (including newline)
    // to the count until we reach the line of the cursor
    for (i, line) in lines.iter().enumerate() {
        count += line.len() as u32 + 1;
        if i + 1 == cursor.line {
            break;
        }
    }
    // Add the column to the count
    count += cursor.column as u32;

    Ok(TextSize::from(count))

}

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

// Trims leading and trailing whitespace from a TextRange
fn trim_whitespace(source_code: &str, range: TextRange) -> TextRange {
    let trimmed_start = range.start();
    let trimmed_end = range.end();

    let start = source_code
        .char_indices()
        .skip_while(|(i, c)| {
            if *i == trimmed_start.into() {
                return c.is_whitespace();
            }
            false
        })
        .next()
        .map(|(i, _)| i)
        .unwrap_or(0);

    let end = source_code
        .char_indices()
        .skip(trimmed_end.into())
        .skip_while(|(_, c)| c.is_whitespace())
        .next()
        .map(|(i, _)| i)
        .unwrap_or(source_code.len());

    TextRange::new(TextSize::from(start as u32), TextSize::from(end as u32))
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    // Utility function to create a temporary file for testing
    fn create_temp_file(content: &str) -> String {
        let temp_file = "test_file.txt";
        let mut file = fs::File::create(temp_file).unwrap();
        writeln!(file, "{}", content).unwrap();
        temp_file.to_string()
    }

    #[test]
    fn test_cursor_to_size_valid() {
        let temp_file = create_temp_file("Hello\nWorld\nRust\n");
        let cursor = Cursor { line: 2, column: 1 }; // Cursor at 'W'
        let size = cursor_to_size(&temp_file, cursor).unwrap();
        assert_eq!(size.raw, 7); // "Hello\n".len() + "W"
    }

    #[test]
    fn test_cursor_to_size_zero_line() {
        let temp_file = create_temp_file("Hello\nWorld\nRust\n");
        let cursor = Cursor { line: 0, column: 1 };
        let result = cursor_to_size(&temp_file, cursor);
        assert!(result.is_err());
    }

    #[test]
    fn test_cursor_to_range_valid() {
        let temp_file = create_temp_file("Hello\nWorld\nRust\n");
        let start_cursor = Cursor { line: 1, column: 1 }; // 'H'
        let end_cursor = Cursor { line: 2, column: 5 }; // 'W'
        let range = cursor_to_range(&temp_file, start_cursor, end_cursor).unwrap();
        assert_eq!(range.start().raw, 0); // "Hello\n".len() = 5 + 1 (newline)
        assert_eq!(range.end().raw, 11); // "Hello\nWorld\n".len() = 11
    }

    #[test]
    fn test_trim_whitespace() {
        let source_code = "   Hello, World!   \n   Rust is fun!   ";
        let range = TextRange::new(TextSize::from(3), TextSize::from(15)); // Range covering "Hello, World!"
        let trimmed_range = trim_whitespace(source_code, range);
        assert_eq!(trimmed_range.start().raw, 0); // Start trimmed to "Hello, World!"
        assert_eq!(trimmed_range.end().raw, 15); // End remains the same
    }

    #[test]
    fn test_trim_whitespace_empty() {
        let source_code = "   \n   ";
        let range = TextRange::new(TextSize::from(0), TextSize::from(3)); // Range covering whitespace
        let trimmed_range = trim_whitespace(source_code, range);
        assert_eq!(trimmed_range.start().raw, 0); // Start trimmed to first non-whitespace
        assert_eq!(trimmed_range.end().raw, 0); // End trimmed to same
    }

    #[test]
    fn test_trim_whitespace_no_trim_needed() {
        let source_code = "Hello, World!";
        let range = TextRange::new(TextSize::from(0), TextSize::from(13)); // Full range
        let trimmed_range = trim_whitespace(source_code, range);
        assert_eq!(trimmed_range.start().raw, 0);
        assert_eq!(trimmed_range.end().raw, 13);
    }
}
