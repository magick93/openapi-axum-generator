use askama::Template;
use openapiv3::OpenAPI;
use serde::Serialize;

#[derive(Template)]
#[template(path = "axum.rs.jinja")]
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
    pub required: bool,
}

impl AxumTemplate<'_> {
    pub fn from_openapi(openapi: &OpenAPI) -> Self {
        let routes = openapi.paths.iter()
            .flat_map(|(path, item)| {
                match item {
                    openapiv3::ReferenceOr::Item(path_item) => {
                        path_item.options.iter()
                            .map(|(method, operation)| Route {
                                path: path.clone(),
                                method: method.to_string().to_uppercase(),
                                handler_name: format!("handle_{}_{}", method, path.replace('/', "_").trim_matches('_')),
                                parameters: operation.parameters.iter()
                                    .filter_map(|param_ref| {
                                        match param_ref {
                                            openapiv3::ReferenceOr::Item(param) => Some(Parameter {
                                                name: param.name.clone(),
                                                param_type: param.schema.as_ref().map_or("String".to_string(), |s| s.to_string()),
                                                required: param.required,
                                            }),
                                            _ => None,
                                        }
                                    })
                                    .collect(),
                                responses: operation.responses.responses.iter()
                                    .map(|(status_code, response)| Response {
                                        status_code: status_code.to_string(),
                                        description: response.description.clone().unwrap_or_default(),
                                        content_type: response.content.keys().next().cloned().unwrap_or_default(),
                                    })
                                    .collect(),
                            })
                            .collect::<Vec<_>>()
                    },
                    _ => Vec::new(),
                }
            })
            .collect();

        let schemas = openapi.components.as_ref()
            .map_or(Vec::new(), |components| {
                components.schemas.iter()
                    .filter_map(|(name, schema_ref)| {
                        match schema_ref {
                            openapiv3::ReferenceOr::Item(schema) => {
                                let fields = schema.schema_data.iter()
                                    .map(|(field_name, field_schema)| SchemaField {
                                        name: field_name.clone(),
                                        field_type: field_schema.to_string(),
                                        required: schema.schema_data.nullable,
                                    })
                                    .collect();
                                Some(Schema {
                                    name: name.clone(),
                                    fields,
                                })
                            },
                            _ => None,
                        }
                    })
                    .collect()
            });

        AxumTemplate {
            openapi,
            routes,
            schemas,
        }
    }
}
