use askama::Template;
use openapiv3::OpenAPI;
use serde::Serialize;
use std::any::Any;

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
                        path_item.iter()
                            .flat_map(|(method, operation)| {
                                std::iter::once((method, operation))
                            })
                            .map(|(method, operation)| Route {
                                path: path.clone(),
                                method: method.to_string().to_uppercase(),
                                handler_name: format!("handle_{}_{}", method, path.replace('/', "_").trim_matches('_')),
                                parameters: operation.parameters.iter()
                                    .filter_map(|param_ref| {
                                        match param_ref {
                                            openapiv3::ReferenceOr::Item(param) => Some(Parameter {
                                                name: param.parameter_data_ref().name.clone(),
                                                param_type: match &param.parameter_data_ref().format {
                                                    openapiv3::ParameterSchemaOrContent::Schema(schema_ref) => {
                                                        match schema_ref {
                                                            openapiv3::ReferenceOr::Item(schema) => format!("{:?}", schema.schema_kind.type_id()),
                                                            _ => "String".to_string(),
                                                        }
                                                    },
                                                    openapiv3::ParameterSchemaOrContent::Content(_) => "Content".to_string(),
                                                },
                                                required: param.parameter_data_ref().required,
                                            }),
                                            _ => None,
                                        }
                                    })
                                    .collect(),
                                responses: operation.responses.responses.iter()
                                    .map(|(status_code, response)| Response {
                                        status_code: status_code.to_string(),
                                        description: match response {
                                            openapiv3::ReferenceOr::Item(resp) => resp.description.clone(),
                                            _ => String::new(),
                                        },
                                        content_type: match response {
                                            openapiv3::ReferenceOr::Item(resp) => resp.content.keys().next().cloned().unwrap_or_default(),
                                            _ => String::new(),
                                        },
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
                                let fields = match &schema.schema_kind {
                                    openapiv3::SchemaKind::Type(openapiv3::Type::Object(obj)) => {
                                        obj.properties.iter()
                                            .map(|(field_name, field_schema)| SchemaField {
                                                name: field_name.clone(),
                                                field_type: field_schema.to_string(),
                                                required: obj.required.contains(field_name),
                                            })
                                            .collect()
                                    },
                                    _ => Vec::new(),
                                };
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
