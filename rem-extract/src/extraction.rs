use std::{
    fs,
    io::{
        self,
        ErrorKind
    },
    path::PathBuf
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

#[derive(Debug, PartialEq, Clone)]
pub struct ExtractionInput {
    pub file_path: String,
    pub output_path: String,
    pub new_fn_name: String,
    pub start_idx: u32,
    pub end_idx: u32,
}


impl ExtractionInput {
    pub fn new(
        file_path: &str,
        output_path: &str,
        new_fn_name: &str,
        start_idx: u32,
        end_idx: u32,
    ) -> Self { ExtractionInput {
            file_path: file_path.to_string(),
            output_path: output_path.to_string(),
            new_fn_name: new_fn_name.to_string(),
            start_idx,
            end_idx,
        }
    }

    pub fn new_absolute(
        file_path: &str,
        output_path: &str,
        new_fn_name: &str,
        start_idx: u32,
        end_idx: u32,
    ) -> Self { ExtractionInput {
            file_path: convert_to_abs_path_buf(file_path).unwrap().as_str().to_string(),
            output_path: convert_to_abs_path_buf(output_path).unwrap().as_str().to_string(),
            new_fn_name: new_fn_name.to_string(),
            start_idx,
            end_idx,
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

// Ensure that the input and output files are not the same
fn input_output_not_same(input: &ExtractionInput) -> Result<(), ExtractionError> {
    if input.file_path == input.output_path {
        return Err(ExtractionError::Io(io::Error::new(
            ErrorKind::InvalidInput,
            "Input and output files cannot be the same",
        )));
    }
    Ok(())
}

// Check if the idx pair is valid
fn check_idx(input: &ExtractionInput) -> Result<(), ExtractionError> {
    if input.start_idx == input.end_idx {
        return Err(ExtractionError::SameIdx);
    } else if input.start_idx > input.end_idx {
        return Err(ExtractionError::InvalidIdxPair);
    }
    if input.start_idx == 0 {
        return Err(ExtractionError::InvalidStartIdx);
    }
    if input.end_idx == 0 {
        return Err(ExtractionError::InvalidEndIdx);
    }
    Ok(())
}

fn verify_input(input: &ExtractionInput) -> Result<(), ExtractionError> {
    // Execute each input validation step one by one
    check_file_exists(&input.file_path)?;
    input_output_not_same(&input)?;
    check_idx(input)?;

    Ok(())
}

// ========================================
// Performs the method extraction
// ========================================

/// Function to extract the code segment based on cursor positions
/// If successful, returns the `PathBuf` to the output file
pub fn extract_method(input: ExtractionInput) -> Result<PathBuf, ExtractionError> {

    // Extract the struct information
    let input_path: &str = &input.file_path;
    let output_path: &str = &input.output_path;
    let callee_name: &str = &input.new_fn_name;
    let start_idx: u32 = input.start_idx;
    let end_idx: u32 = input.end_idx;

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
        start_idx,
        end_idx,
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

