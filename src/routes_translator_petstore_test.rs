#[cfg(test)]
mod tests {
    use crate::RoutesTranslator;
    use openapiv3::OpenAPI;
    use std::fs;
    use std::process::Command;

    fn run_generation_test(test_data_path: &str) {
        // Clean up any existing generated files
        crate::TestUtils::cleanup_generated_files()
            .expect("Failed to clean up generated files");

        // Run the CLI command
        let status = Command::new("cargo")
            .args(["run", "--", "--input", test_data_path, "--output", "gen/"])
            .status()
            .expect("Failed to run CLI command");

        assert!(status.success(), "CLI command failed");

        // Verify output files were generated
        assert!(fs::metadata("gen/axum_server.rs").is_ok(), "axum_server.rs not generated");
    }

    #[test]
    fn test_petstore_generation() {
        run_generation_test("./src/test_data/petstore.json");
        assert!(fs::metadata("gen/src/api/handlers.rs").is_ok(), "handlers.rs not generated");
        assert!(fs::metadata("gen/src/pets/handlers.rs").is_ok(), "pets/handlers.rs not generated");
    }

    #[test]
    fn test_uspto_generation() {
        run_generation_test("./src/test_data/uspto.json");
        // Add USPTO-specific assertions here
    }

    fn load_test_data(path: &str) -> OpenAPI {
        let json = fs::read_to_string(path)
            .expect(&format!("Failed to read {}", path));
        serde_json::from_str(&json)
            .expect(&format!("Failed to parse {}", path))
    }

    fn verify_route_basics(routes: &[crate::routes::models::Route], path: &str, method: &str, handler_name: &str) {
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

    #[test]
    fn test_handler_signatures() {
        // Test petstore handlers
        let petstore_handlers = fs::read_to_string("gen/src/pets/handlers.rs")
            .expect("Failed to read generated petstore handlers.rs");

        // Verify GET /pets/{petId} handler uses correct name and TypedPath
        assert!(
            petstore_handlers.contains("pub async fn handle_get_pets_petid("),
            "Handler name should be handle_get_pets_petid"
        );
        assert!(petstore_handlers.contains("petId: String,"));
        assert!(petstore_handlers.contains(") -> Result<JsonResponse<Pet>, StatusCode> {"));

        // Verify other petstore handlers
        assert!(petstore_handlers.contains("pub async fn handle_get_pets("));
        assert!(petstore_handlers.contains("limit: Option<i32>,"));
        assert!(petstore_handlers.contains("pub async fn handle_post_pets("));

        // Test uspto handlers
        let uspto_handlers = fs::read_to_string("gen/src/api/handlers.rs")
            .expect("Failed to read generated uspto handlers.rs");

        // Verify GET / handler
        assert!(
            uspto_handlers.contains("pub async fn handle_get_root("),
            "Handler name should be handle_get_root"
        );
        assert!(uspto_handlers.contains(") -> Result<JsonResponse<DataSetList>, StatusCode> {"));

        // Verify GET /{dataset}/{version}/fields handler
        assert!(
            uspto_handlers.contains("pub async fn handle_get_dataset_version_fields("),
            "Handler name should be handle_get_dataset_version_fields"
        );
        assert!(uspto_handlers.contains("dataset: String,"));
        assert!(uspto_handlers.contains("version: String,"));
        assert!(uspto_handlers.contains(") -> Result<JsonResponse<String>, StatusCode> {"));

        // Verify POST /{dataset}/{version}/records handler
        assert!(
            uspto_handlers.contains("pub async fn handle_post_dataset_version_records("),
            "Handler name should be handle_post_dataset_version_records"
        );
        assert!(uspto_handlers.contains("dataset: String,"));
        assert!(uspto_handlers.contains("version: String,"));
        assert!(uspto_handlers.contains("criteria: String,"));
        assert!(uspto_handlers.contains("start: Option<i32>,"));
        assert!(uspto_handlers.contains("rows: Option<i32>,"));
        assert!(uspto_handlers.contains(") -> Result<JsonResponse<Vec<HashMap<String, Value>>>, StatusCode> {"));
    }

    #[test]
    fn test_generated_structs() {
        // Read generated handlers file
        let handlers_content = fs::read_to_string("gen/src/pets/handlers.rs")
            .expect("Failed to read generated handlers.rs");

        // Verify Pet struct
        assert!(handlers_content.contains("pub struct Pet {"));
        assert!(handlers_content.contains("pub id: i64,"));
        assert!(handlers_content.contains("pub name: String,"));
        assert!(handlers_content.contains("pub tag: Option<String>,"));
        assert!(handlers_content.contains("#[derive(Debug, Serialize, Deserialize, ToSchema, TypedPath)]"));
        
        // Verify Pets struct
        assert!(handlers_content.contains("pub struct Pets {"));
        
        // Verify Error struct
        assert!(handlers_content.contains("pub struct Error {"));
        assert!(handlers_content.contains("pub code: i64,"));
        assert!(handlers_content.contains("pub message: String,"));
    }

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
}
