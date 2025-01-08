use askama::Template;
use crate::filters::{is_pet_id_route, snake_case, sanitize_handler_name};

use openapiv3::OpenAPI;
use serde::Serialize;

mod filters;
mod routes_translator;
#[cfg(test)]
mod routes_translator_petstore_test;
#[cfg(test)]
mod routes_translator_test;
#[cfg(test)]
mod routes_translator_uspto_test;
mod schemas_translator;

use routes_translator::RoutesTranslator;
use schemas_translator::SchemasTranslator;

#[derive(Template)]
#[template(path = "axum_utoipa.rs.jinja", escape = "none")]
pub struct AxumTemplate<'a> {
    pub openapi: &'a OpenAPI,
    pub routes: Vec<RouteWithoutTags>,
    pub schemas: Vec<Schema>,
    pub filters: &'a [(&'a str, &'a dyn Fn(&str) -> askama::Result<String>)],
}

impl<'a> AxumTemplate<'a> {
    fn new(openapi: &'a OpenAPI, routes: Vec<RouteWithoutTags>, schemas: Vec<Schema>) -> Self {
        Self {
            openapi,
            routes,
            schemas,
            filters: &[
                ("is_pet_id_route", &is_pet_id_route),
                ("snake_case", &snake_case),
                ("sanitize_handler_name", &sanitize_handler_name)
            ],
        }
    }
}

#[derive(Serialize)]
pub struct RouteWithoutTags {
    pub path: String,
    pub method: String,
    pub handler_name: String,
    pub schema: SchemaRef,
    pub parameters: Vec<Parameter>,
    pub path_parameters: Vec<String>, // Changed from Vec<Parameter> to Vec<String>
    pub responses: Vec<Response>,
    pub tag: String,
}

#[derive(Template)]
#[template(path = "mod.rs.jinja", escape = "none")]
pub struct ModTemplate {
    pub modules: Vec<String>,
}

#[derive(Serialize)]
pub struct Route {
    pub path: String,
    pub method: String,
    pub handler_name: String,
    pub schema: SchemaRef,
    pub parameters: Vec<Parameter>,
    pub path_parameters: Vec<String>, // Changed from Vec<Parameter> to Vec<String>
    pub responses: Vec<Response>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
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

#[derive(Serialize, Clone)]
pub struct Schema {
    pub name: String,
    pub path: String,
    pub fields: Vec<SchemaField>,
}

#[derive(Serialize, Clone)]
pub struct SchemaField {
    pub name: String,
    pub field_type: String,
    pub rust_type: String,
    pub required: bool,
}

impl AxumTemplate<'_> {
    pub fn from_openapi<'a>(openapi: &'a OpenAPI) -> Vec<(String, String)> {
        let routes_translator = RoutesTranslator::new();
        let schemas_translator = SchemasTranslator::new();
        
        let routes = routes_translator.translate(openapi);
        let schemas = schemas_translator.translate(openapi);
        
        // Group routes by their base path
        let mut modules = Vec::new();
        let mut module_routes = std::collections::HashMap::new();
        
        for route in routes {
            let path_parts: Vec<&str> = route.path.split('/').filter(|s| !s.is_empty()).collect();
            let module_path = path_parts[0].to_string();
            
            if !modules.contains(&module_path) {
                modules.push(module_path.clone());
            }
            
            module_routes
                .entry(module_path.clone())
                .or_insert_with(Vec::new)
                .push(route);
        }

        // Generate files for each module
        let mut files = Vec::new();
        
        // Generate handler files
        for (module, routes) in module_routes {
            let routes_without_tags = routes.into_iter().map(|route| RouteWithoutTags {
                path: route.path,
                method: route.method,
                handler_name: route.handler_name,
                schema: route.schema,
                parameters: route.parameters,
                path_parameters: route.path_parameters,
                responses: route.responses,
                tag: route.tags.first().cloned().unwrap_or_else(|| "Default".to_string()),
            }).collect();
            
            // Collect all path parameters from routes
            let template = AxumTemplate::new(
                openapi,
                routes_without_tags,
                schemas.clone()
            );
            
            let content = template.render().unwrap();
            files.push((format!("src/{}/handlers.rs", module), content));
        }
        
        // Generate mod.rs files
        let mod_template = ModTemplate { modules };
        let mod_content = mod_template.render().unwrap();
        files.push(("src/mod.rs".to_string(), mod_content));
        
        files
    }
}
