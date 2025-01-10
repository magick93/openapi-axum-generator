use std::fs;
use std::path::Path;
use schemars;
use typify::{TypeSpace, TypeSpaceSettings};

pub fn generate_types_from_schemas() -> Result<(), Box<dyn std::error::Error>> {
    let schema_dir = Path::new("src/test_data/schemas");
    
    // Create output directory if it doesn't exist
    let output_dir = Path::new("src/generated_types");
    fs::create_dir_all(output_dir)?;

    // Process each JSON schema file
    for entry in fs::read_dir(schema_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            // Read the schema file
            let schema_content = fs::read_to_string(&path)?;
            
            // Create a TypeSpace with settings
            let mut settings = TypeSpaceSettings::default();
            settings
                .with_derive("serde::Serialize".to_string())
                .with_derive("serde::Deserialize".to_string());
            let mut type_space = TypeSpace::new(&settings);
            
            // Parse schema content into RootSchema
            let root_schema = serde_json::from_str::<schemars::schema::RootSchema>(&schema_content)?;
            
            // Add the schema content
            type_space.add_root_schema(root_schema)?;
            
            // Generate output file path
            let output_path = output_dir.join(
                path.file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string() + ".rs"
            );
            
            // Write the generated types
            let tokens = type_space.to_stream();
            let generated = tokens.to_string();
            fs::write(output_path, generated)?;
        }
    }
    
    Ok(())
}
