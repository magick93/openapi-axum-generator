use openapiv3::ReferenceOr;
use serde::Serialize;

/// Represents a route parameter
#[derive(Serialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
    pub required: bool,
}

/// Represents a route response
#[derive(Serialize)]
pub struct Response {
    pub status_code: String,
    pub description: String,
    pub content_type: String,
}

/// Represents a generated route
#[derive(Serialize)]
pub struct Route {
    pub path: String,
    pub method: String,
    pub handler_name: String,
    pub schema: SchemaRef,
    pub parameters: Vec<Parameter>,
    pub path_parameters: Vec<String>,
    pub responses: Vec<Response>,
    pub tags: Vec<String>,
}

/// Reference to a schema used in a route
#[derive(Serialize)]
pub struct SchemaRef {
    pub name: String,
}
