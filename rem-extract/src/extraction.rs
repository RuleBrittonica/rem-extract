
use std::{
    fs,
    io::{
        self,
        ErrorKind,
        Read,
    },
    path::PathBuf,
};

use ra_ap_project_model::{
    CargoConfig,
    ProjectWorkspace,
    ProjectManifest,
};

use ra_ap_ide::{
    Analysis,
    AnalysisHost,
};

use ra_ap_ide_assists::Assist;

use ra_ap_vfs::AbsPathBuf;

use crate::{
    error::ExtractionError,
    extraction_utils::{
        apply_extract_function,
        convert_to_abs_path_buf,
        filter_extract_function_assist,
        get_assists,
        get_cargo_config,
        get_cargo_toml,
        get_manifest_dir,
        load_project_manifest,
        load_project_workspace,
        load_workspace_data,
        run_analysis,
    },
};

/// A 1-indexed struct to represent the position of a Cursor as a human user
/// sees it.
#[derive(Debug, PartialEq, Clone)]
pub struct Cursor {
    pub line: usize, // Line in file, 1-indexed
    pub column: usize, // Column in line, 1-indexed
}

impl Cursor {
    pub fn new(line: usize, column: usize) -> Self {
        Cursor { line, column }
    }

    /// Cursor is 1 indexed - this method takes that into account
    pub fn to_offset(&self, file_path: &AbsPathBuf) -> Result<u32, ExtractionError> {
        let mut file = fs::File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let lines: Vec<&str> = content.lines().collect();

        let line_idx = self.line - 1;
        if line_idx >= lines.len() {
            return Err(ExtractionError::InvalidLineRange);
        }

        let mut offset: u32 = 0;
        for i in 0..line_idx {
            offset += lines[i].len() as u32 + 1; // Adding 1 for the newline character
        }

        let cursor_line = lines[line_idx];

        let column_idx = self.column - 1;
        if column_idx > cursor_line.len() {
            println!("{} > {}", self.column, cursor_line.len());
            return Err(ExtractionError::InvalidColumnRange);
        }
        // Add the byte length of characters on the cursor's line up to the column
        offset += cursor_line[..column_idx].len() as u32;

        Ok(offset)
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
    let callee_name: &str = &input.new_fn_name;

    // Convert the input and output path to an `AbsPathBuf`
    let input_abs_path: AbsPathBuf = convert_to_abs_path_buf(input_path).unwrap();
    let output_abs_path: AbsPathBuf = convert_to_abs_path_buf(output_path).unwrap();
    // print!("Output Path: {:?}", output_abs_path);

    // Verify the input data
    verify_input(&input)?;

    let manifest_dir: PathBuf = get_manifest_dir(
        &PathBuf::from(input_abs_path.as_str())
    )?;
    let cargo_toml: AbsPathBuf = get_cargo_toml( &manifest_dir );
    // println!("Cargo.toml {:?}", cargo_toml);

    let project_manifest: ProjectManifest = load_project_manifest( &cargo_toml );
    // println!("Project Manifest {:?}", project_manifest);

    let cargo_config: CargoConfig = get_cargo_config( &project_manifest );
    // println!("Cargo Config {:?}", cargo_config);

    let workspace: ProjectWorkspace = load_project_workspace( &project_manifest, &cargo_config );
    // println!("Project Workspace {:?}", workspace);

    let (db, vfs) = load_workspace_data(workspace, &cargo_config);
    // println!("Database {:?}", db);
    // println!("VFS {:?}", vfs);

    let analysis_host: AnalysisHost = AnalysisHost::with_database( db );
    let analysis: Analysis = run_analysis( analysis_host );
    // println!("Analysis {:?}", analysis);

    // Parse the cursor positions into the range
    let range: (u32, u32) = (
        start_cursor.to_offset(&input_abs_path)?,
        end_cursor.to_offset(&input_abs_path)?
    );

    let assists: Vec<Assist> = get_assists(&analysis, &vfs, &input_abs_path, range);

    let assist: Assist = filter_extract_function_assist(assists)?;

    apply_extract_function(
        &assist,
        &input_abs_path,
        &output_abs_path,
        &vfs,
        &callee_name,
    );

    Ok(PathBuf::new())
}

