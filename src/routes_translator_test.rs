use super::routes_translator::RoutesTranslator;
use openapiv3::{
    Components, OpenAPI, Operation, Parameter, ParameterData, ParameterSchemaOrContent, PathItem,
    ReferenceOr, Response, Schema, SchemaKind, StatusCode, Type,
};

fn create_test_openapi() -> OpenAPI {
    OpenAPI {
        openapi: "3.0.0".to_string(),
        paths: Default::default(),
        components: Some(Components {
            schemas: Default::default(),
            ..Default::default()
        }),
        ..Default::default()
    }
}

#[test]
fn test_translate_route_with_tags() {
    let mut openapi = create_test_openapi();
    openapi.paths.paths.insert(
        "/test".to_string(),
        ReferenceOr::Item(PathItem {
            get: Some(Operation {
                operation_id: Some("testOperation".to_string()),
                parameters: vec![],
                responses: Default::default(),
                tags: vec!["pets".to_string()],
                ..Default::default()
            }),
            ..Default::default()
        }),
    );

    let translator = RoutesTranslator::new();
    let routes = translator.translate(&openapi);

    assert_eq!(routes.len(), 1);
    let route = &routes[0];
    assert_eq!(route.tags, vec!["pets"]);
}

#[test]
fn test_translate_basic_route() {
    let mut openapi = create_test_openapi();
    openapi.paths.paths.insert(
        "/test".to_string(),
        ReferenceOr::Item(PathItem {
            get: Some(Operation {
                operation_id: Some("testOperation".to_string()),
                parameters: vec![],
                responses: Default::default(),
                ..Default::default()
            }),
            ..Default::default()
        }),
    );

    let translator = RoutesTranslator::new();
    let routes = translator.translate(&openapi);

    assert_eq!(routes.len(), 1);
    let route = &routes[0];
    assert_eq!(route.path, "/test");
    assert_eq!(route.method, "GET");
    assert_eq!(route.handler_name, "handle_get_test");
}

#[test]
fn test_translate_route_with_parameters() {
    let mut openapi = create_test_openapi();
    openapi.paths.paths.insert(
        "/test/{id}".to_string(),
        ReferenceOr::Item(PathItem {
            get: Some(Operation {
                operation_id: Some("testOperation".to_string()),
                parameters: vec![ReferenceOr::Item(Parameter::Query {
                    parameter_data: ParameterData {
                        name: "id".to_string(),
                        description: None,
                        required: true,
                        deprecated: Some(false),
                        format: ParameterSchemaOrContent::Schema(ReferenceOr::Item(Schema {
                            schema_kind: SchemaKind::Type(Type::String(Default::default())),
                            schema_data: Default::default(),
                        })),
                        example: None,
                        examples: Default::default(),
                        explode: Some(false),
                        extensions: Default::default(),
                    },
                    allow_empty_value: Some(false),
                    style: openapiv3::QueryStyle::Form,
                    allow_reserved: false,
                })],
                responses: Default::default(),
                ..Default::default()
            }),
            ..Default::default()
        }),
    );

    let translator = RoutesTranslator::new();
    let routes = translator.translate(&openapi);

    assert_eq!(routes.len(), 1);
    let route = &routes[0];
    assert_eq!(route.parameters.len(), 1);
    let param = &route.parameters[0];
    assert_eq!(param.name, "id");
    assert_eq!(param.param_type, "Type(String(StringType { format: Empty, pattern: None, enumeration: [], min_length: None, max_length: None }))");
    assert!(param.required);
}

#[test]
fn test_translate_route_with_responses() {
    let mut openapi = create_test_openapi();
    openapi.paths.paths.insert(
        "/test".to_string(),
        ReferenceOr::Item(PathItem {
            get: Some(Operation {
                operation_id: Some("testOperation".to_string()),
                parameters: vec![],
                responses: openapiv3::Responses {
                    responses: vec![
                        (
                            StatusCode::Code(200),
                            ReferenceOr::Item(Response {
                                description: "Success".to_string(),
                                content: Default::default(),
                                ..Default::default()
                            }),
                        ),
                        (
                            StatusCode::Code(404),
                            ReferenceOr::Item(Response {
                                description: "Not Found".to_string(),
                                content: Default::default(),
                                ..Default::default()
                            }),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        }),
    );

    let translator = RoutesTranslator::new();
    let routes = translator.translate(&openapi);

    assert_eq!(routes.len(), 1);
    let route = &routes[0];
    assert_eq!(route.responses.len(), 2);

    let success_response = &route.responses[0];
    assert_eq!(success_response.status_code, "200");
    assert_eq!(success_response.description, "Success");

    let not_found_response = &route.responses[1];
    assert_eq!(not_found_response.status_code, "404");
    assert_eq!(not_found_response.description, "Not Found");
}

#[test]
fn test_translate_route_with_reserved_keyword_parameter() {
    let mut openapi = create_test_openapi();
    openapi.paths.paths.insert(
        "/test/{type}".to_string(),
        ReferenceOr::Item(PathItem {
            get: Some(Operation {
                operation_id: Some("testOperation".to_string()),
                parameters: vec![ReferenceOr::Item(Parameter::Query {
                    parameter_data: ParameterData {
                        name: "type".to_string(),
                        description: None,
                        required: true,
                        deprecated: Some(false),
                        format: ParameterSchemaOrContent::Schema(ReferenceOr::Item(Schema {
                            schema_kind: SchemaKind::Type(Type::String(Default::default())),
                            schema_data: Default::default(),
                        })),
                        example: None,
                        examples: Default::default(),
                        explode: Some(false),
                        extensions: Default::default(),
                    },
                    allow_empty_value: Some(false),
                    style: openapiv3::QueryStyle::Form,
                    allow_reserved: false,
                })],
                responses: Default::default(),
                ..Default::default()
            }),
            ..Default::default()
        }),
    );

    let translator = RoutesTranslator::new();
    let routes = translator.translate(&openapi);

    assert_eq!(routes.len(), 1);
    let route = &routes[0];
    assert_eq!(route.parameters.len(), 1);
    let param = &route.parameters[0];
    assert_eq!(param.name, "r#type"); // Expect escaped keyword
    assert_eq!(param.param_type, "Type(String(StringType { format: Empty, pattern: None, enumeration: [], min_length: None, max_length: None }))");
    assert!(param.required);
}
