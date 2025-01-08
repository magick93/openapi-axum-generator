#[cfg(test)]
mod tests {
    use crate::routes_translator::RoutesTranslator;
    use openapiv3::OpenAPI;
    use std::fs;
    use std::process::Command;

    #[test]
    fn test_cli_generation() {
        // Run the CLI command
        let status = Command::new("cargo")
            .args(["run", "--", "--input", "./src/test_data/petstore.json", "--output", "gen/"])
            .status()
            .expect("Failed to run CLI command");

        assert!(status.success(), "CLI command failed");

        // Verify output files were generated
        assert!(fs::metadata("gen/axum_server.rs").is_ok(), "axum_server.rs not generated");
        assert!(fs::metadata("gen/src/api/handlers.rs").is_ok(), "handlers.rs not generated");
        assert!(fs::metadata("gen/src/pets/handlers.rs").is_ok(), "pets/handlers.rs not generated");
    }

    #[test]
    fn test_handler_signatures() {
        let handlers_content = fs::read_to_string("gen/src/pets/handlers.rs")
            .expect("Failed to read generated handlers.rs");

        // Verify GET /pets/{petId} handler uses correct name and TypedPath
        assert!(
            handlers_content.contains("pub async fn handle_get_pets_petid("),
            "Handler name should be handle_get_pets_petid"
        );
        assert!(handlers_content.contains("petId: String,"));
        assert!(handlers_content.contains(") -> Result<JsonResponse<Pet>, StatusCode> {"));

        // Verify other handlers
        assert!(handlers_content.contains("pub async fn handle_get_pets("));
        assert!(handlers_content.contains("limit: Option<i32>,"));
        
        assert!(handlers_content.contains("pub async fn handle_post_pets("));
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
