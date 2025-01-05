use std::{fs, path::Path};
use askama::Template;

#[derive(Debug)]
struct Route {
    path: String,
    method: String,
    handler: String,
}

#[derive(Debug)]
struct Model {
    name: String,
    fields: Vec<ModelField>,
}

#[derive(Debug)]
struct ModelField {
    name: String,
    field_type: String,
}

#[derive(Debug)]
struct Handler {
    name: String,
    path_params: Option<String>,
    path_params_type: Option<String>,
    query_params: Option<String>,
    query_params_type: Option<String>,
    body: Option<String>,
    body_type: Option<String>,
    response: String,
}

#[derive(Template)]
#[template(path = "main.rs.j2")]
struct MainTemplate<'a> {
    routes: &'a Vec<Route>,
}

#[derive(Template)]
#[template(path = "handler.rs.j2")]
struct HandlerTemplate<'a> {
    handler: &'a Handler,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Askama templates are compiled at build time
    // No runtime template engine initialization needed

    // Create output directory
    let output_dir = "generated";
    fs::create_dir_all(output_dir)?;

    // Example data - this would come from OpenAPI spec parsing
    let routes = vec![
        Route {
            path: "/users".to_string(),
            method: "get".to_string(),
            handler: "get_users".to_string(),
        },
        Route {
            path: "/users/{id}".to_string(),
            method: "get".to_string(),
            handler: "get_user".to_string(),
        },
    ];

    let models = vec![
        Model {
            name: "User".to_string(),
            fields: vec![
                ModelField {
                    name: "id".to_string(),
                    field_type: "u64".to_string(),
                },
                ModelField {
                    name: "name".to_string(),
                    field_type: "String".to_string(),
                },
            ],
        },
    ];

    // Generate main.rs
    let main_rs = MainTemplate { routes: &routes }.render()
        .map_err(|e| format!("Failed to render main.rs: {}", e))?;
    fs::write(Path::new(output_dir).join("main.rs"), main_rs)
        .map_err(|e| format!("Failed to write main.rs: {}", e))?;

    // Generate handlers
    for route in &routes {
        let handler_rs = HandlerTemplate {
            handler: &Handler {
                name: route.handler.clone(),
                path_params: None,
                path_params_type: None,
                query_params: None,
                query_params_type: None,
                body: None,
                body_type: None,
                response: "User".to_string(),
            }
        }.render()
        .map_err(|e| format!("Failed to render {}.rs: {}", route.handler, e))?;
        fs::write(
            Path::new(output_dir).join(format!("{}.rs", route.handler)),
            handler_rs,
        ).map_err(|e| format!("Failed to write {}.rs: {}", route.handler, e))?;
    }

    println!("Successfully generated Axum server in {}", output_dir);
    Ok(())
}
