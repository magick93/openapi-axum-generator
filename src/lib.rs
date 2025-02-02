use askama::Template;
use log::{debug, error, info};

use openapiv3::OpenAPI;
use serde::Serialize;

pub mod file_utils;
pub mod filters;
pub mod functions_translator;
pub mod routes;
pub mod routes_translator;
pub mod schema_generator;
pub mod schemas_translator;
pub mod test_utils;

pub use schema_generator::generate_types_from_schemas;

#[cfg(test)]
mod routes_translator_petstore_test;
#[cfg(test)]
mod routes_translator_test;
#[cfg(test)]
mod routes_translator_uspto_test;

use functions_translator::FunctionSignature;
use routes_translator::RoutesTranslator;
use schemas_translator::SchemasTranslator;

#[derive(Template)]
#[template(path = "axum_utoipa.rs.jinja", escape = "none")]
pub struct AxumTemplate<'a> {
    pub openapi: &'a OpenAPI,
    pub routes: Vec<RouteWithoutTags>,
    pub functions: Vec<FunctionSignature>,
    pub folders: std::collections::HashSet<String>,
    pub schemas: Vec<Schema>,
}

impl<'a> AxumTemplate<'a> {
    fn new(
        openapi: &'a OpenAPI,
        routes: Vec<RouteWithoutTags>,
        schemas: Vec<Schema>,
        functions: Vec<FunctionSignature>,
    ) -> Self {
        Self {
            openapi,
            routes,
            schemas,
            functions,
            folders: std::collections::HashSet::new(),
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
    pub path_parameters: Vec<String>,
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
    pub path_parameters: Vec<String>,
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
    pub fn from_openapi(openapi: &OpenAPI) -> Vec<(String, String)> {
        info!("Starting OpenAPI translation");
        let routes_translator = RoutesTranslator::new();
        let schemas_translator = SchemasTranslator::new();
        let functions_translator = FunctionSignature::new();

        debug!("Initialized translators");

        let routes = routes_translator.translate(openapi);
        debug!("Translated {} routes", routes.len());
        
        let schemas = schemas_translator.translate(openapi);
        debug!("Translated {} schemas", schemas.len());

        let mut modules = Vec::new();
        let mut module_routes = std::collections::HashMap::new();
        debug!("Starting module organization");
        

        for route in routes {
            let path_parts: Vec<&str> = route.path.split('/').filter(|s| !s.is_empty()).collect();
            let module_path = path_parts[0].to_string();
            debug!("Processing route: {} -> module: {}", route.path, module_path);

            if !modules.contains(&module_path) {
                modules.push(module_path.clone());
            }

            module_routes
                .entry(module_path.clone())
                .or_insert_with(Vec::new)
                .push(route);
        }

        let mut files = Vec::new();
        info!("Generating handler files for {} modules", module_routes.len());

        for (module, routes) in module_routes {
            debug!("Generating handlers for module: {}", module);
            let routes_without_tags = routes
                .into_iter()
                .map(|route| RouteWithoutTags {
                    path: route.path,
                    method: route.method,
                    handler_name: route.handler_name,
                    schema: route.schema,
                    parameters: route.parameters,
                    path_parameters: route.path_parameters,
                    responses: route.responses,
                    tag: route
                        .tags
                        .first()
                        .cloned()
                        .unwrap_or_else(|| "Default".to_string()),
                })
                .collect();

            let functions = functions_translator.translate(openapi);

            let mut folders = std::collections::HashSet::new();

            for function in &functions {
                folders.insert(function.folder.clone());
            }

            let mut template = AxumTemplate::new(
                openapi,
                routes_without_tags,
                schemas.clone(),
                functions,
            );

            template.folders = folders;

            let content = match template.render() {
                Ok(content) => {
                    debug!("Successfully rendered template for module: {}", module);
                    content
                }
                Err(e) => {
                    error!("Failed to render template for module {}: {}", module, e);
                    continue;
                }
            };
            files.push((format!("src/{}/handlers.rs", module), content));
        }

        let mod_template = ModTemplate { modules };
        let mod_content = match mod_template.render() {
            Ok(content) => {
                debug!("Successfully rendered mod.rs template");
                content
            }
            Err(e) => {
                error!("Failed to render mod.rs template: {}", e);
                return files;
            }
        };
        files.push(("src/mod.rs".to_string(), mod_content));

        info!("Completed OpenAPI translation, generated {} files", files.len());
        files
    }
}
