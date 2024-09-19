use std::fs;
use syn::{
    parse_file,
    File
};
use colored::*;
use quote::ToTokens;
use crate::{
    extract_method,
    ExtractionInput,
    error::ExtractionError,
};

struct TestFile {
    input_file: String, // Just the name of the file. It is assumed the file is in ./input, and there is a corresponding file in ./correct_output
    start_line: usize,
    end_line: usize,
}

fn parse_and_compare_ast(output_file_path: &str, expected_file_path: &str) -> Result<bool, ExtractionError> {
    let output_content = fs::read_to_string(output_file_path)?;
    let expected_content = fs::read_to_string(expected_file_path)?;

    let output_ast: File = parse_file(&output_content)?;
    let expected_ast: File = parse_file(&expected_content)?;

    // Convert both ASTs back into token streams for comparison
    let output_tokens = output_ast.into_token_stream().to_string();
    let expected_tokens = expected_ast.into_token_stream().to_string();

    Ok(output_tokens == expected_tokens)
}

pub fn test() {
    let test_files: Vec<TestFile> = vec![
        TestFile {
            input_file: "simple_example.rs".to_string(),
            start_line: 1,
            end_line: 3,
        },
        TestFile {
            input_file: "decode.rs".to_string(),
            start_line: 51,
            end_line: 73,
        },
        TestFile {
            input_file: "complex_example.rs".to_string(),
            start_line: 2,
            end_line: 5,
        },
    ];

    let start_time: std::time::Instant = std::time::Instant::now();

    for test_file in test_files {
        let input_file_path: String = format!("./input/{}", test_file.input_file);
        let output_file_path: String = format!("./output/{}", test_file.input_file);
        let expected_file_path: String = format!("./correct_output/{}", test_file.input_file);

        let input = ExtractionInput {
            file_path: input_file_path.clone(),
            output_file_path: output_file_path.clone(),
            start_line: test_file.start_line,
            end_line: test_file.end_line,
            new_fn_name: "new_function".to_string(),
        };

        // Call the extraction method and handle errors
        let extraction_result: Result<String, ExtractionError> = extract_method(input);

        // Measure time taken for extraction
        let elapsed_time: std::time::Duration = start_time.elapsed();
        let elapsed_time_secs: f64 = elapsed_time.as_secs_f64();
        let elapsed_time_str: String = if elapsed_time_secs < 1.0 {
            format!("{:.2}ms", elapsed_time_secs * 1000.0)
        } else {
            format!("{:.2}s", elapsed_time_secs)
        };

        let test_name: &str = test_file.input_file.trim_end_matches(".rs");
        let mut extraction_status: String = "FAILED".red().to_string();
        let mut comparison_status: String = "N/A".to_string(); // Default to not applicable

        if extraction_result.is_ok() {
            extraction_status = "PASSED".green().to_string();

            // Compare the output file with the expected file's AST
            match parse_and_compare_ast(&output_file_path, &expected_file_path) {
                Ok(is_identical) => {
                    if is_identical {
                        comparison_status = "PASSED".green().to_string();
                    } else {
                        comparison_status = "FAILED".red().to_string();
                    }
                }
                Err(e) => {
                    comparison_status = format!("Error: {}", e).red().to_string();
                }
            }
        } else if let Err(e) = extraction_result {
            extraction_status = format!("FAILED: {}", e).red().to_string();
        }

        println!("{} | {}: {} in {}", extraction_status, comparison_status, test_name, elapsed_time_str);
    }

    // Total elapsed time
    let total_elapsed_time: std::time::Duration = start_time.elapsed();
    let total_elapsed_time_secs: f64 = total_elapsed_time.as_secs_f64();
    let total_elapsed_time_str: String = if total_elapsed_time_secs < 1.0 {
        format!("{:.2}ms", total_elapsed_time_secs * 1000.0)
    } else {
        format!("{:.2}s", total_elapsed_time_secs)
    };

    println!("------------------------------------------------------------------");
    println!("Total time: {}", total_elapsed_time_str);
}
