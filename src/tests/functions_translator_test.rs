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
}
