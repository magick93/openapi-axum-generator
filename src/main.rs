use clap::{Arg, Command};
use openapiv3::OpenAPI;
use serde_json::from_str;
use std::{fs, path::Path, io};

/// Helper function to write content to a file, creating parent directories if needed
fn write_file_with_path(path: &Path, content: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // Create output directory
    fs::create_dir_all(output_dir)?;

    // Load and parse OpenAPI spec
    let spec_str = fs::read_to_string(input_file)?;
    let openapi_spec: OpenAPI = from_str(&spec_str)?;

    // Generate files
    let files = openapi_axum_generator::AxumTemplate::from_openapi(&openapi_spec)?;

    // Write all generated files using helper function
    for (file_path, content) in files {
        let full_path = Path::new(output_dir).join(file_path);
        write_file_with_path(&full_path, &content)?;
    }

    println!("Successfully generated Axum server code in {}", output_dir);
    Ok(())
}
