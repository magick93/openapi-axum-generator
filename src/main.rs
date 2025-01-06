use std::{fs, path::Path};
use clap::{Arg, Command};
use serde_json::from_str;
use openapiv3::OpenAPI;

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

    let input_file = matches.get_one::<String>("input").expect("input is required");
    let output_dir = matches.get_one::<String>("output").expect("output is required");

    // Create output directory
    fs::create_dir_all(output_dir)?;

    // Load and parse OpenAPI spec
    let spec_str = fs::read_to_string(input_file)?;
    let openapi_spec: OpenAPI = from_str(&spec_str)?;

    // Convert OpenAPI spec to template data
    let template = openapi_axum_generator::AxumTemplate::from_openapi(&openapi_spec);

    // Render template and write output
    let output = template.render()
        .map_err(|e| format!("Template rendering failed: {}", e))?;
    fs::write(Path::new(output_dir).join("axum_server.rs"), output)?;

    println!("Successfully generated Axum server code in {}", output_dir);
    Ok(())
}
