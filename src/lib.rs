use askama::Template;
use openapiv3::OpenAPI;
use serde::Serialize;

mod filters;
mod routes_translator;
#[cfg(test)]
mod routes_translator_petstore_test;
#[cfg(test)]
mod routes_translator_test;
mod schemas_translator;

use routes_translator::RoutesTranslator;
use schemas_translator::SchemasTranslator;

#[derive(Template)]
#[template(path = "axum_utoipa.rs.jinja")]
pub struct AxumTemplate<'a> {
    pub openapi: &'a OpenAPI,
    pub routes: Vec<Route>,
    pub schemas: Vec<Schema>,
}

#[derive(Serialize)]
pub struct Route {
    pub path: String,
    pub method: String,
    pub handler_name: String,
    pub parameters: Vec<Parameter>,
    pub responses: Vec<Response>,
    pub schema: SchemaRef,
}

#[derive(Serialize)]
pub struct SchemaRef {
    pub name: String,
}

#[derive(Serialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
    pub required: bool,
}

#[derive(Serialize)]
pub struct Response {
    pub status_code: String,
    pub description: String,
    pub content_type: String,
}

#[derive(Serialize)]
pub struct Schema {
    pub name: String,
    pub fields: Vec<SchemaField>,
}

#[derive(Serialize)]
pub struct SchemaField {
    pub name: String,
    pub field_type: String,
    pub rust_type: String,
    pub required: bool,
}

impl AxumTemplate<'_> {
    pub fn from_openapi<'a>(openapi: &'a OpenAPI) -> AxumTemplate<'a> {
        let routes_translator = RoutesTranslator::new();
        let schemas_translator = SchemasTranslator::new();

        AxumTemplate {
            openapi,
            routes: routes_translator.translate(openapi),
            schemas: schemas_translator.translate(openapi),
        }
    }
}
