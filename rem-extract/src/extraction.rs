use rem_utils::fmt_file;

use std::{
    convert, fs, io::{self, ErrorKind}, path::PathBuf
};

use ra_ap_project_model::{
    CargoConfig,
    ProjectWorkspace,
    ProjectManifest,
};

use ra_ap_ide::{
    Analysis,
    AnalysisHost,
    DiagnosticsConfig,
    FilePosition,
    FileRange,
    RootDatabase,
    SingleResolve,
    TextRange,
    TextSize,
    TextEdit,
    SnippetEdit,
    SourceChange,
};

use ra_ap_ide_db::{
    imports::insert_use::{
        ImportGranularity,
        InsertUseConfig,
        PrefixKind,
    }, rename, SnippetCap
};

use ra_ap_ide_assists::{
    Assist,
    AssistConfig,
    AssistKind,
    AssistResolveStrategy,
};

use ra_ap_vfs::{
    AbsPathBuf,
    VfsPath,
    Vfs,
    FileId,
};

use ra_ap_load_cargo::{
    LoadCargoConfig,
    ProcMacroServerChoice,
    load_workspace,
};

use crate::{
    error::ExtractionError,
    extraction_utils::{
        convert_to_abs_path_buf,
    },
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

fn check_cursor_not_equal(input: &ExtractionInput) -> Result<(), ExtractionError> {
    if input.start_cursor == input.end_cursor {
        return Err(ExtractionError::SameCursor);
    }

    Ok(())
}

fn verify_input(input: &ExtractionInput) -> Result<(), ExtractionError> {
    // Execute each input validation step one by one
    check_file_exists(&input.file_path)?;
    check_line_numbers(input)?;
    check_columns(input)?;
    check_cursor_not_equal(input)?;

    Ok(())
}

// ========================================
// Performs the method extraction
// ========================================

/// Function to extract the code segment based on cursor positions
/// If successful, returns the `PathBuf` to the output file
pub fn extract_method(input: ExtractionInput) -> Result<PathBuf, ExtractionError> {
    // Get the cursor positions
    let start_cursor: Cursor = input.clone().start_cursor;
    let end_cursor: Cursor = input.clone().end_cursor;

    // Get info about the files
    let input_path: &str = &input.file_path;
    let output_path: &str = &input.output_path;
    let new_fn_name: &str = &input.new_fn_name;

    // Convert the output path to an `AbsPathBuf`
    let output_abs_path: AbsPathBuf = convert_to_abs_path_buf(output_path).unwrap();
    // print!("Output Path: {:?}", output_abs_path);

    verify_input(&input)?;



    Ok(PathBuf::new())
}

