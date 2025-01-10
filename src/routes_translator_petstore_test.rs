#[cfg(test)]
mod tests {
    use crate::RoutesTranslator;
    use crate::test_utils::TestUtils;
    
    use openapiv3::OpenAPI;
    use std::fs;
    use std::process::Command;

    fn run_generation_test(test_data_path: &str) {
        // Ensure gen directory exists
        fs::create_dir_all("gen").expect("Failed to create gen directory");
        
        // Clean up any existing generated files
        TestUtils::cleanup_generated_files()
            .expect("Failed to clean up generated files");

        // Run the CLI command
        let output = Command::new("cargo")
            .args(["run", "--", "--input", test_data_path, "--output", "gen/"])
            .output()
            .expect("Failed to run CLI command");

        if !output.status.success() {
            eprintln!("CLI command failed with status: {}", output.status);
            eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            panic!("CLI command failed");
        }

        // Verify output files were generated
        // assert!(fs::metadata("gen/axum_server.rs").is_ok(), "axum_server.rs not generated");
    }

    #[test]
    fn test_petstore_generation() {
        init();
        run_generation_test("./src/test_data/petstore.json");
        // assert!(fs::metadata("gen/src/api/handlers.rs").is_ok(), "handlers.rs not generated");
        assert!(fs::metadata("gen/src/pets/handlers.rs").is_ok(), "pets/handlers.rs not generated");
    }

    // #[test]
    // fn test_uspto_generation() {
    //     run_generation_test("./src/test_data/uspto.json");
    //     // Add USPTO-specific assertions here
    // }

    fn load_test_data(path: &str) -> OpenAPI {
        let json = fs::read_to_string(path)
            .expect(&format!("Failed to read {}", path));
        serde_json::from_str(&json)
            .expect(&format!("Failed to parse {}", path))
    }

    fn verify_route_basics(routes: &Vec<crate::Route>, path: &str, method: &str, handler_name: &str) {
        let route = routes
            .iter()
            .find(|r| r.path == path && r.method == method)
            .unwrap_or_else(|| panic!("Route {} {} not found", method, path));
        
        assert_eq!(route.handler_name, handler_name);
    }

    #[test]
    fn test_route_translation() {
        let translator = RoutesTranslator::new();
        
        // Test petstore routes
        let petstore = load_test_data("./src/test_data/petstore.json");
        let petstore_routes = translator.translate(&petstore);
        assert_eq!(petstore_routes.len(), 3);
        verify_route_basics(&petstore_routes, "/pets", "GET", "handle_get_pets");
        verify_route_basics(&petstore_routes, "/pets", "POST", "handle_post_pets");
        verify_route_basics(&petstore_routes, "/pets/{petId}", "GET", "handle_get_pets_petid");

        // Test uspto routes
        let uspto = load_test_data("./src/test_data/uspto.json");
        let uspto_routes = translator.translate(&uspto);
        assert!(uspto_routes.len() > 0);
    }

    

    // #[test]
    // fn test_generated_structs() {
    //     // Read generated handlers file
    //     let handlers_content = fs::read_to_string("gen/src/pets/handlers.rs")
    //         .expect("Failed to read generated handlers.rs");

    //     // Verify Pet struct
    //     assert!(handlers_content.contains("pub struct Pet {"));
    //     assert!(handlers_content.contains("pub id: i64,"));
    //     assert!(handlers_content.contains("pub name: String,"));
    //     assert!(handlers_content.contains("pub tag: Option<String>,"));
    //     assert!(handlers_content.contains("#[derive(Debug, Serialize, Deserialize, ToSchema, TypedPath)]"));
        
    //     // Verify Pets struct
    //     assert!(handlers_content.contains("pub struct Pets {"));
        
    //     // Verify Error struct
    //     assert!(handlers_content.contains("pub struct Error {"));
    //     assert!(handlers_content.contains("pub code: i64,"));
    //     assert!(handlers_content.contains("pub message: String,"));
    // }

    #[test]
    fn test_translate_petstore_routes() {
        // Load petstore OpenAPI spec
        let petstore_json = fs::read_to_string("src/test_data/petstore.json")
            .expect("Failed to read petstore.json");
        let openapi: OpenAPI =
            serde_json::from_str(&petstore_json).expect("Failed to parse petstore.json");

        let translator = RoutesTranslator::new();
        let routes = translator.translate(&openapi);

        // Verify we have all expected routes
        assert_eq!(routes.len(), 3);

        // Test /pets GET route
        let list_pets = routes
            .iter()
            .find(|r| r.path == "/pets" && r.method == "GET")
            .unwrap();
        assert_eq!(list_pets.handler_name, "handle_get_pets");
        assert_eq!(list_pets.parameters.len(), 1);
        assert_eq!(list_pets.parameters[0].name, "limit");
        assert_eq!(list_pets.parameters[0].param_type, "Type(Integer(IntegerType { format: Item(Int32), multiple_of: None, exclusive_minimum: false, exclusive_maximum: false, minimum: None, maximum: None, enumeration: [] }))");
        assert!(!list_pets.parameters[0].required);
        assert_eq!(list_pets.responses.len(), 1);

        // Test /pets POST route
        let create_pets = routes
            .iter()
            .find(|r| r.path == "/pets" && r.method == "POST")
            .unwrap();
        assert_eq!(create_pets.handler_name, "handle_post_pets");
        assert_eq!(create_pets.parameters.len(), 0);
        assert_eq!(create_pets.responses.len(), 1);

        // Test /pets/{petId} GET route
        let show_pet = routes
            .iter()
            .find(|r| r.path == "/pets/{petId}" && r.method == "GET")
            .unwrap();
        assert_eq!(show_pet.handler_name, "handle_get_pets_petid");
        assert_eq!(show_pet.parameters.len(), 1);
        assert_eq!(show_pet.parameters[0].name, "petId");
        assert!(show_pet.parameters[0].param_type.contains("Type(String"));
        assert!(show_pet.parameters[0].required);
        assert_eq!(show_pet.responses.len(), 1);
    }

    fn init() {
        let _ = env_logger::builder()
            .target(env_logger::Target::Stdout)
            .filter_level(log::LevelFilter::Trace)
            .is_test(true)
            .try_init();
    }

}
