use clap::{Arg, Command};
use std::{fs, io, path::Path};

mod file_utils;
mod schema_generator;
mod reporter;

use crate::reporter::Reporter;
use crate::file_utils::openapi_from_file;

/// Helper function to write content to a file, creating parent directories if needed
fn write_file_with_path(path: &Path, content: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init();
    let matches = Command::new("openapi-axum-generator")
        .version("0.1.0")
        .about("Generates Axum server code from OpenAPI specification")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Path to OpenAPI JSON file")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("DIR")
                .help("Output directory for generated files")
                .required(false),
        )
        .get_matches();

    let input_file = matches
        .get_one::<String>("input")
        .expect("input is required");
    let output_dir = matches
        .get_one::<String>("output")
        .expect("output is required");

    // log the input and output
    log::debug!("### Input file: {:?}", input_file);
    
    // Create output directory
    fs::create_dir_all(output_dir)?;

    // Load and parse OpenAPI spec
    let openapi_spec = openapi_from_file(input_file)?;

    // Initialize reporter
    let mut reporter = Reporter::new();
    
    // Generate schema types
    schema_generator::generate_types_from_schemas(output_dir, &mut reporter)?;

    // Generate files and get report
    let (files, _report) = openapi_axum_generator::AxumTemplate::from_openapi(&openapi_spec);

    // Write all generated files using helper function
    for (file_path, content) in files {
        let full_path = Path::new(output_dir).join(file_path);
        write_file_with_path(&full_path, &content)?;
        
        // Track generated file with actual line count
        let line_count = content.lines().count();
        reporter.record_file(full_path, line_count);
    }

    println!("Successfully generated Axum server code in {}", output_dir);
    println!("\n{}", reporter.print_report());
    Ok(())
}

fn init() {
    let _ = env_logger::builder()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Trace)
        .is_test(true)
        .try_init();
}
