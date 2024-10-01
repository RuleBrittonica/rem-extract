//! Utility functions for the rem-extract crate.
//! At some point these will be merged into rem-utils.

use crate::extraction::{
    Cursor,
};

use std::path::PathBuf;

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



// TODO
pub fn cursor_to_offset( cursor: &Cursor ) -> u32 {
   0 as u32
}

/// TODO
/// Returns the path to the manifest directory of the given file
/// The manifest directory is the directory containing the Cargo.toml file
pub fn get_manifest_dir( path: &PathBuf ) -> PathBuf {


    PathBuf::new()
}

/// Given a `PathBuf` to a folder, returns the `AbsPathBuf` to the `Cargo.toml`
/// file in that folder.
pub fn get_cargo_toml( manifest_dir: &PathBuf ) -> AbsPathBuf {
    AbsPathBuf::try_from(
        manifest_dir
            .join( "Cargo.toml" )
            .to_str()
            .unwrap()
    ).unwrap()
}

/// Loads as `ProjectManifest` from the given `AbsPathBuf` to a `Cargo.toml` file.
pub fn load_project_manifest( cargo_toml: &AbsPathBuf ) -> ProjectManifest {
    ProjectManifest::from_manifest_file(
        cargo_toml.clone()
    ).unwrap()
}

pub fn progress( _message: String ) -> () {
    // println!( "{}", _message );
}

/// Loads a project workspace from a `ProjectManifest` and `CargoConfig`
pub fn load_project_workspace(
    project_manifest: &ProjectManifest,
    cargo_config: &CargoConfig,
) -> ProjectWorkspace {
    ProjectWorkspace::load(
        project_manifest.clone(),
        cargo_config,
        &progress
    ).unwrap()
}

/// Loads a `RootDatabase` containing from a `ProjectWorkspace` and `CargoConfig`
pub fn load_workspace_data(
    workspace: ProjectWorkspace,
    cargo_config: &CargoConfig,
) -> (
    RootDatabase,
    Vfs
) {
    let load_cargo_config: LoadCargoConfig = LoadCargoConfig {
        load_out_dirs_from_check: true,
        with_proc_macro_server: ProcMacroServerChoice::None,
        prefill_caches: false,
    };

    let (db,
        vfs,
        _proc_macro
    ) = load_workspace(
        workspace,
        &cargo_config.extra_env,
        &load_cargo_config
    ).unwrap();

    (db, vfs)
}

/// Runs the analysis on an AnalysisHost. A wrapper around `AnalysisHost::analysis`
fn run_analysis( host: AnalysisHost ) -> Analysis {

    let analysis: Analysis = host.analysis();

    analysis
}

/// Gets a list of available assists for a given file and range
fn get_assists (
    analysis: &Analysis,
    manifest_dir: &PathBuf,
    vfs: &Vfs,
    path_components: &Vec<&str>, // Vec of path components, e.g. [ "src", "main.rs" ]
    range: (u32, u32), // Tuple of start and end offsets
) -> Vec<Assist> {

    // Build out the AssistConfig
    let snippet_cap_: Option<SnippetCap> = None;
    let allowed_assists: Vec<AssistKind> = vec![
        AssistKind::RefactorExtract,
    ];

    let insert_use_: InsertUseConfig = InsertUseConfig {
        granularity: ImportGranularity::Preserve,
        enforce_granularity: false,
        prefix_kind: PrefixKind::ByCrate,
        group: false,
        skip_glob_imports: false,
    };

    let assist_config: AssistConfig = AssistConfig {
        snippet_cap: snippet_cap_,
        allowed: Some(allowed_assists),
        insert_use: insert_use_,
        prefer_no_std: false,
        prefer_prelude: false,
        prefer_absolute: false,
        assist_emit_must_use: false,
        term_search_fuel: 2048, // * NFI what this is
        term_search_borrowck: false,
    };

    // Build out the DiagnosticsConfig
    let diagnostics_config: DiagnosticsConfig = DiagnosticsConfig::test_sample(); // TODO This may need to be specific to the program

    // Build out the ResolveStrategy
    // FIXME: This is currently bugged it seems - Both extract_variable and extract_function are being returned
    let resolve: AssistResolveStrategy = AssistResolveStrategy::Single(
        SingleResolve {
            assist_id: "extract_function".to_string(),
            assist_kind: AssistKind::RefactorExtract,
        }
    );

    // Build out the FileRange
    let mut file_path: PathBuf = manifest_dir.clone();
    for component in path_components {
        file_path = file_path.join( component );
    }
    let vfs_path: VfsPath = VfsPath::new_real_path(
        file_path
            .to_string_lossy()
            .to_string(),
    );

    let file_id_: FileId = vfs.file_id( &vfs_path ).unwrap();
    let range_: TextRange = TextRange::new(
        TextSize::try_from( range.0 ).unwrap(),
        TextSize::try_from( range.1 ).unwrap(),
    );

    let frange: FileRange = FileRange {
        file_id: file_id_,
        range: range_,
    };

    // Call the assists_with_fixes method
    let assists: Vec<Assist> = analysis.assists_with_fixes(
        &assist_config,
        &diagnostics_config,
        resolve,
        frange
    ).unwrap();

    assists
}

/// Filter the list of assists to only be the extract_function assist
/// FIXME This is a hack to get around the fact that the resolve strategy is bugged
/// and is returning both extract_variable and extract_function
pub fn filter_extract_function_assist( assists: Vec<Assist> ) -> &Assist {
    assists
        .iter()
        .find( |assist| assist.label == "Extract into function" )
        .unwrap()
}

/// Converts a `VfsPath` to a `PathBuf`
pub fn vfs_to_pathbuf( vfs_path: &VfsPath ) -> PathBuf {
    let path_str = vfs_path.to_string();
    // println!("{}", path_str);
    PathBuf::from( path_str )
}

pub fn apply_extract_function(
    assist: &Assist,
    manifest_dir: &PathBuf,
    vfs: &Vfs,
    path_components: &Vec<&str>, // Vec of path components, e.g. [ "src", "main.rs" ]
    out_components: &Vec<&str>, // Vec of path components, e.g. [ "output", "simple.rs" ]
) -> PathBuf {
    // Copy the source file to the output directory
    let mut in_file_path: PathBuf = manifest_dir.clone();
    for component in path_components {
        in_file_path = in_file_path.join( component );
    }
    let vfs_in_path: VfsPath = VfsPath::new_real_path(
        in_file_path
            .to_string_lossy()
            .to_string(),
    );

    // The output is derived from the project directory
    let mut out_file_path: PathBuf = get_manifest_dir( "./" );
    for component in out_components {
        out_file_path = out_file_path.join( component );
    }
    let vfs_out_path: VfsPath = VfsPath::new_real_path(
        out_file_path
            .to_string_lossy()
            .to_string(),
    );

    copy_file_vfs( &vfs_in_path, &vfs_out_path );

    // From here, extract the source change, but apply it to the copied file
    let src_change: SourceChange = assist.source_change
        .as_ref()
        .unwrap()
        .clone();
    let in_file_id: FileId = vfs.file_id( &vfs_in_path ).unwrap();
    let (text_edit, maybe_snippet_edit) = src_change.get_source_and_snippet_edit(
        in_file_id
    ).unwrap();
    let text_edit: TextEdit = text_edit.clone();
    let maybe_snippet_edit: Option<SnippetEdit> = maybe_snippet_edit.clone();

    apply_edits(
        &vfs_out_path,
        text_edit,
        maybe_snippet_edit,
    );

    // Rename the function from fun_name to NEW_FUNCTION_NAME using a search and
    // replace on the output file
    rename_function(
        &vfs_out_path,
        "fun_name",
        NEW_FUNCTION_NAME,
    );

    // Return the output file path
    PathBuf::from( vfs_out_path.to_string() )
}

// Apply a text edit.
// Then apply the snippet edit if it is present
fn apply_edits(
    vfs_path: &VfsPath,
    text_edit: TextEdit,
    maybe_snippet_edit: Option<SnippetEdit>,
) -> () {
    let path: PathBuf = vfs_to_pathbuf( vfs_path );
    let mut text: String = std::fs::read_to_string( &path ).unwrap();
    text_edit.apply( &mut text );
    match maybe_snippet_edit {
        Some( snippet_edit ) => {
            snippet_edit.apply( &mut text );
        },
        None => (),
    }
    std::fs::write( &path, text ).unwrap();
}

// Rename a function in a file using a search and replace
fn rename_function(
    vfs_path: &VfsPath,
    old_name: &str,
    new_name: &str,
) -> () {
    let path: PathBuf = vfs_to_pathbuf( vfs_path );
    let mut text: String = std::fs::read_to_string( &path ).unwrap();
    let old_name: String = old_name.to_string();
    let new_name: String = new_name.to_string();
    text = text.replace( &old_name, &new_name );
    std::fs::write( &path, text ).unwrap();
}