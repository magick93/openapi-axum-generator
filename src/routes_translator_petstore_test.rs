use super::routes_translator::RoutesTranslator;
use openapiv3::OpenAPI;
use std::fs;

#[test]
fn test_translate_petstore_routes() {
    // Load petstore OpenAPI spec
    let petstore_json = fs::read_to_string("src/test_data/petstore.json")
        .expect("Failed to read petstore.json");
    let openapi: OpenAPI = serde_json::from_str(&petstore_json)
        .expect("Failed to parse petstore.json");

    let translator = RoutesTranslator::new();
    let routes = translator.translate(&openapi);

    // Verify we have all expected routes
    assert_eq!(routes.len(), 3);
    
    // Test /pets GET route
    let list_pets = routes.iter().find(|r| r.path == "/pets" && r.method == "GET").unwrap();
    assert_eq!(list_pets.handler_name, "handle_get_pets");
    assert_eq!(list_pets.parameters.len(), 1);
    assert_eq!(list_pets.parameters[0].name, "limit");
    assert_eq!(list_pets.parameters[0].param_type, "Type(Integer { format: Int32 })");
    assert!(!list_pets.parameters[0].required);
    assert_eq!(list_pets.responses.len(), 2);

    // Test /pets POST route
    let create_pets = routes.iter().find(|r| r.path == "/pets" && r.method == "POST").unwrap();
    assert_eq!(create_pets.handler_name, "handle_post_pets");
    assert_eq!(create_pets.parameters.len(), 0);
    assert_eq!(create_pets.responses.len(), 2);

    // Test /pets/{petId} GET route
    let show_pet = routes.iter().find(|r| r.path == "/pets/{petId}" && r.method == "GET").unwrap();
    assert_eq!(show_pet.handler_name, "handle_get_pets_petId");
    assert_eq!(show_pet.parameters.len(), 1);
    assert_eq!(show_pet.parameters[0].name, "petId");
    assert_eq!(show_pet.parameters[0].param_type, "Type(String {})");
    assert!(show_pet.parameters[0].required);
    assert_eq!(show_pet.responses.len(), 2);
}
