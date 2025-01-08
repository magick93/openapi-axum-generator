use crate::routes::models::Route;
use openapiv3::OpenAPI;
use std::fs;
use std::path::Path;
use tera::{Tera, Context};

pub struct OpenAPITranslator {
    tera: Tera,
}

impl OpenAPITranslator {
    pub fn new() -> Self {
        let mut tera = Tera::new("templates/**/*.jinja").unwrap();
        tera.autoescape_on(vec![]);
        Self { tera }
    }

    pub fn generate_server_code(&self, openapi: &OpenAPI, output_dir: &str) -> Result<(), anyhow::Error> {
        let routes = self.translate_routes(openapi);
        let schemas = self.translate_schemas(openapi);

        // Create output directory
        let output_path = Path::new(output_dir);
        fs::create_dir_all(output_path)?;

        // Generate axum_server.rs
        let mut context = Context::new();
        context.insert("routes", &routes);
        context.insert("schemas", &schemas);
        
        let server_code = self.tera.render("axum_server.rs.jinja", &context)?;
        fs::write(output_path.join("axum_server.rs"), server_code)?;

        Ok(())
    }

    fn translate_routes(&self, openapi: &OpenAPI) -> Vec<Route> {
        // TODO: Implement route translation
        vec![]
    }

    fn translate_schemas(&self, openapi: &OpenAPI) -> Vec<Schema> {
        // TODO: Implement schema translation
        vec![]
    }
}

#[derive(Debug)]
struct Schema {
    name: String,
    fields: Vec<SchemaField>,
}

#[derive(Debug)]
struct SchemaField {
    name: String,
    field_type: String,
}
