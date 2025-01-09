#[cfg(test)]
mod tests {
    use crate::functions_translator::{
    FunctionSignature, 
    ParameterSignature, 
    ParameterLocation,
    RequestBodySignature,
    ResponseSignature
};
    use crate::file_utils;
    use openapiv3::ReferenceOr;
    use std::fmt::Debug;
    use std::path::PathBuf;

    fn init() {
        let _ = env_logger::builder()
            .target(env_logger::Target::Stdout)
            .filter_level(log::LevelFilter::Trace)
            .is_test(true)
            .try_init();
    }

    #[test]
    fn test_function_signature_creation() {
        let sig = FunctionSignature::new();
        assert_eq!(sig.fn_name, "");
        assert_eq!(sig.http_method, "GET");
        assert!(sig.params.is_empty());
        assert!(sig.responses.is_empty());
    }

    #[test]
    fn test_parameter_handling() {
        let mut sig = FunctionSignature::new();
        sig.params.push(ParameterSignature {
            name: "id".to_string(),
            rust_type: "i32".to_string(),
            location: ParameterLocation::Path,
            description: Some("Item ID".to_string()),
        });

        assert_eq!(sig.params.len(), 1);
        let param = &sig.params[0];
        assert_eq!(param.name, "id");
        assert_eq!(param.rust_type, "i32");
    }

    #[test]
    fn test_request_body_handling() {
        let mut sig = FunctionSignature::new();
        sig.request_body = Some(RequestBodySignature {
            rust_type: "CreateTodo".to_string(),
            description: Some("New todo item".to_string()),
        });

        assert!(sig.request_body.is_some());
        let body = sig.request_body.as_ref().unwrap();
        assert_eq!(body.rust_type, "CreateTodo");
    }

    #[test]
    fn test_response_handling() {
        let mut sig = FunctionSignature::new();
        sig.responses.push(ResponseSignature {
            status: 200,
            description: Some("Success".to_string()),
            rust_type: Some("Vec<Todo>".to_string()),
        });

        assert_eq!(sig.responses.len(), 1);
        let response = &sig.responses[0];
        assert_eq!(response.status, 200);
        assert_eq!(response.rust_type.as_ref().unwrap(), "Vec<Todo>");
    }

    #[test]
    fn test_translate_function_with_minimal_spec() {
        init();

        
        let openapi = file_utils::create_minimal_openapi();
        let sig = FunctionSignature::new();
        let result = sig.translate(&openapi);

        //log results length
        log::debug!("Result Length: {:?}", result.len());

        //print results
        for r in &result {
            
            log::debug!("Function Name: {:?}", r.fn_name);
            log::debug!("Path         : {:?}", r.path);
        }
        
        assert_eq!(result.len(), 1);
        let translated = &result[0];
        // log translated
        // log::debug!("Translated: {:?}", translated.doc_comment);
        // log::debug!("openapi.info.description: {:?}", openapi.info.description);
        // assert_eq!(translated.doc_comment, openapi.info.description);
    }

    #[test]
    fn test_translate_function_with_real_spec() {
        init();
        let path = PathBuf::from("src/test_data/openapi.json");
        let openapi = file_utils::openapi_from_file(&path).unwrap();
        
        // Verify basic OpenAPI structure
        assert_eq!(openapi.openapi, "3.0.0");
        assert!(!openapi.info.title.is_empty());
        assert!(!openapi.info.version.is_empty());
        
        let sig = FunctionSignature::new();
        let result = sig.translate(&openapi);

        // log results length
        log::debug!("Result Length: {:?}", result.len());

        // print results
        for r in &result {
            log::debug!("Function Name: {:?} {:?}", r.fn_name, r.http_method);
            log::debug!("Path         : {:?}", r.path);
        }
        
        // Verify translation results
        assert!(!result.is_empty());
        let translated = &result[0];
        assert!(translated.doc_comment.is_some());
        assert!(!translated.fn_name.is_empty());
        assert!(!translated.path.is_empty());
    }

    #[test]
    fn test_translate_with_invalid_spec() {
        let path = PathBuf::from("src/test_data/invalid.json");
        let result = file_utils::openapi_from_file(&path);
        assert!(result.is_err());
    }

    // #[test]
    // fn test_v4_exclusion_in_path() {
    //     init();
        
    //     // Test v4 at start of path
    //     let mut sig1 = FunctionSignature::new();
    //     sig1.path = "/v4/test".to_string();
    //     let result1 = sig1.translate(&file_utils::create_minimal_openapi());
    //     assert_eq!(result1[0].path, "/test");

    //     // Test v4 in middle of path
    //     let mut sig2 = FunctionSignature::new();
    //     sig2.path = "/api/v4/test".to_string();
    //     let result2 = sig2.translate(&file_utils::create_minimal_openapi());
    //     assert_eq!(result2[0].path, "/api/test");

    //     // Test v4 at end of path
    //     let mut sig3 = FunctionSignature::new();
    //     sig3.path = "/test/v4".to_string();
    //     let result3 = sig3.translate(&file_utils::create_minimal_openapi());
    //     assert_eq!(result3[0].path, "/test");

    //     // Test multiple v4 occurrences
    //     let mut sig4 = FunctionSignature::new();
    //     sig4.path = "/v4/api/v4/test/v4".to_string();
    //     let result4 = sig4.translate(&file_utils::create_minimal_openapi());
    //     assert_eq!(result4[0].path, "/api/test");

    //     // Test path without v4 remains unchanged
    //     let mut sig5 = FunctionSignature::new();
    //     sig5.path = "/api/test".to_string();
    //     let result5 = sig5.translate(&file_utils::create_minimal_openapi());
    //     assert_eq!(result5[0].path, "/api/test");
    // }

    #[test]
    fn test_v4_exclusion_in_folder() {
        init();
        
        // Test v4 at start of folder name
        let mut sig1 = FunctionSignature::new();
        sig1.folder = "v4_test".to_string();


        let min_openapi = file_utils::create_minimal_openapi();
        
        //TODO - get tags from openapi and log. So we can check we are settting test data correctly
        // Get and verify tags from the minimal openapi spec
        let path_item = min_openapi.paths.paths.first().unwrap().1;
        if let ReferenceOr::Item(path_item) = path_item {
            if let Some(get_op) = &path_item.get {
                log::debug!("Operation Tags: {:?}", get_op.tags);
                assert!(!get_op.tags.is_empty(), "Tags should be set in minimal openapi spec");
                assert_eq!(get_op.tags[0], "v4", "First tag should be 'v4'");
            }
        }

        let result1 = sig1.translate(&min_openapi);
        log::debug!("Result1: {:?}", result1[0].folder);
        assert_eq!(result1[0].folder, "test");

        // Test v4 in middle of folder name
        let mut sig2 = FunctionSignature::new();
        sig2.folder = "api_v4_test".to_string();
        let result2 = sig2.translate(&file_utils::create_minimal_openapi());
        assert_eq!(result2[0].folder, "api_test");

        // Test v4 at end of folder name
        let mut sig3 = FunctionSignature::new();
        sig3.folder = "test_v4".to_string();
        let result3 = sig3.translate(&file_utils::create_minimal_openapi());
        assert_eq!(result3[0].folder, "test");

        // Test multiple v4 occurrences
        let mut sig4 = FunctionSignature::new();
        sig4.folder = "v4_api_v4_test_v4".to_string();
        let result4 = sig4.translate(&file_utils::create_minimal_openapi());
        assert_eq!(result4[0].folder, "api_test");

        // Test folder without v4 remains unchanged
        let mut sig5 = FunctionSignature::new();
        sig5.folder = "api_test".to_string();
        let result5 = sig5.translate(&file_utils::create_minimal_openapi());
        assert_eq!(result5[0].folder, "api_test");
    }
}
