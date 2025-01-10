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

        log::debug!("Result Length: {:?}", result.len());

        for r in &result {
            log::debug!("Function Name: {:?}", r.fn_name);
            log::debug!("Path         : {:?}", r.path);
        }
        
        assert_eq!(result.len(), 1);
        let translated = &result[0];
        assert_eq!(translated.fn_name, "get");
    }

    #[test]
    fn test_translate_function_with_real_spec() {
        init();
        let path = PathBuf::from("src/test_data/openapi.json");
        let openapi = file_utils::openapi_from_file(&path).unwrap();
        
        assert_eq!(openapi.openapi, "3.0.0");
        assert!(!openapi.info.title.is_empty());
        assert!(!openapi.info.version.is_empty());
        
        let sig = FunctionSignature::new();
        let result = sig.translate(&openapi);

        log::debug!("Result Length: {:?}", result.len());

        for r in &result {
            log::debug!("Function Name: {:?} {:?}", r.fn_name, r.http_method);
            log::debug!("Path         : {:?}", r.path);
        }
        
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

    #[test]
    fn test_v4_exclusion_in_folder() {
        init();
        
        let mut sig1 = FunctionSignature::new();
        sig1.folder = "v4_test".to_string();

        let min_openapi = file_utils::create_minimal_openapi();
        
        let path_item = min_openapi.paths.paths.first().unwrap().1;
        if let ReferenceOr::Item(path_item) = path_item {
            if let Some(get_op) = &path_item.get {
                log::debug!("Operation Tags: {:?}", get_op.tags);
                assert!(!get_op.tags.is_empty());
                assert_eq!(get_op.tags[0], "v4");
            }
        }

        let result1 = sig1.translate(&min_openapi);
        log::debug!("Result1: {:?}", result1[0].folder);
        assert_eq!(result1[0].folder, "tag_test");

        let mut sig2 = FunctionSignature::new();
        sig2.folder = "api_test".to_string();

        let mut min_openapi = file_utils::create_minimal_openapi();
        let path_item = min_openapi.paths.paths.first_mut().unwrap().1;
        if let ReferenceOr::Item(path_item) = path_item {
            if let Some(get_op) = &mut path_item.get {
                get_op.tags = vec!["v4".to_string()];
            }
        }

        let result2 = sig2.translate(&min_openapi);
        log::debug!("Result2: {:?}", result2[0].folder);
        assert_eq!(result2[0].folder, "default");
    }

    #[test]
    fn test_to_snake_case() {
        init();
        let cases = vec![
            ("get_api_v4_network_validators_validatorsByClusterHash_clusterHash", 
             "get_network_validators_validators_by_cluster_hash_cluster_hash"),
            ("get_api_v4_network_validators", "get_network_validators"),
            ("get_api_v4_network", "get_network"),
            ("search_controller_search", "search_controller"),
            ("get_api_v4", "get"),
            ("get_api", "get"),
            ("get", "get"),
            ("", ""),
        ];

        for (input, expected) in cases {
            let result = FunctionSignature::to_snake_case(input);
            log::debug!("Result: {:?}", result);
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }
}
