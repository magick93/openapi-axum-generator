//! Utilities for working with OpenAPI specifications

use axum::http::status;
use openapiv3::OpenAPI;
use std::error::Error;
use std::fs;
use std::path::Path;
use serde_json::from_str;
use serde_yaml::from_reader;

/// Reads an OpenAPI specification from a file
/// 
/// # Arguments
/// * `input_file` - Path to the OpenAPI specification file
/// 
/// # Returns
/// Result containing the parsed OpenAPI specification or an error
pub fn openapi_from_file<P: AsRef<Path>>(input_file: P) -> Result<OpenAPI, Box<dyn Error>> {
    let path = input_file.as_ref();
    let spec_str = fs::read_to_string(path)?;
    
    // Try parsing as JSON first, then fall back to YAML
    if let Ok(spec) = from_str::<OpenAPI>(&spec_str) {
        return Ok(spec);
    }
    
    // If JSON parsing failed, try YAML
    let file = fs::File::open(path)?;
    let spec = from_reader(file)?;
    Ok(spec)
}

/// Creates a minimal OpenAPI specification for testing
pub fn create_minimal_openapi() -> OpenAPI {
    OpenAPI {
        openapi: "3.0.0".to_string(),
        info: openapiv3::Info {
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            description: Some("This is a test API".to_string()),
            ..Default::default()
        },
        // add a sample path
        paths: {
            let mut paths = openapiv3::Paths::default();
            paths.paths.insert(
                "/test".to_string(),
                openapiv3::ReferenceOr::Item(openapiv3::PathItem {
                    get: Some(openapiv3::Operation {
                        tags: vec!["v4".to_string(),"tag_test".to_string(), "sample".to_string(), "api".to_string()],
                        operation_id: Some("get_api_v4_{network}_validators_validatorsByClusterHash_{clusterHash}".to_string()),
                        responses: {
                            let mut responses = openapiv3::Responses::default();
                            responses.responses.insert(
                                openapiv3::StatusCode::Code(status::StatusCode::OK.as_u16()),
                                openapiv3::ReferenceOr::Item(openapiv3::Response {
                                    description: "OK".to_string(),
                                    ..Default::default()
                                }),
                            );
                            responses
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            );
            paths
        },
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_openapi_from_file() {
        let path = PathBuf::from("src/test_data/petstore.json");
        let result = openapi_from_file(&path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_minimal_openapi() {
        let spec = create_minimal_openapi();
        assert_eq!(spec.openapi, "3.0.0");
        assert_eq!(spec.info.title, "Test API");
    }
}
