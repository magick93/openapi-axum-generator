use openapiv3::{OpenAPI, ParameterSchemaOrContent, ReferenceOr};

use super::{Parameter as RouteParameter, Response as RouteResponse, Route};

/// List of Rust keywords that need to be escaped
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn",
];

/// Escape Rust keywords by prefixing with r#
fn escape_rust_keyword(name: &str) -> String {
    if RUST_KEYWORDS.contains(&name) {
        format!("r#{}", name)
    } else {
        name.to_string()
    }
}

pub struct RoutesTranslator;

impl RoutesTranslator {
    pub fn new() -> Self {
        Self
    }

    pub fn translate(&self, openapi: &OpenAPI) -> Vec<Route> {
        openapi
            .paths
            .iter()
            .flat_map(|(path, item)| match item {
                ReferenceOr::Item(path_item) => path_item
                    .iter()
                    .flat_map(|(method, operation)| std::iter::once((method, operation)))
                    .map(|(method, operation)| {
                        // Get schema name from request body or first response
                        let schema_name = operation
                            .request_body
                            .as_ref()
                            .and_then(|rb| match rb {
                                ReferenceOr::Item(body) => {
                                    body.content.values().next().and_then(|media| {
                                        match &media.schema {
                                            Some(ReferenceOr::Item(schema)) => {
                                                schema.schema_data.title.clone()
                                            }
                                            _ => None,
                                        }
                                    })
                                }
                                _ => None,
                            })
                            .or_else(|| {
                                operation
                                    .responses
                                    .responses
                                    .values()
                                    .next()
                                    .and_then(|resp| match resp {
                                        ReferenceOr::Item(r) => {
                                            r.content.values().next().and_then(|media| match &media
                                                .schema
                                            {
                                                Some(ReferenceOr::Item(schema)) => {
                                                    schema.schema_data.title.clone()
                                                }
                                                _ => None,
                                            })
                                        }
                                        _ => None,
                                    })
                            })
                            .unwrap_or_else(|| "DefaultSchema".to_string());

                        // Extract path parameters from the path string
                        let path_parameters: Vec<String> = path
                            .split('/')
                            .filter(|segment| segment.starts_with('{') && segment.ends_with('}'))
                            .map(|segment| {
                                let param_name = segment.trim_matches(|c| c == '{' || c == '}');
                                escape_rust_keyword(param_name)
                            })
                            .collect();

                        // Generate handler name
                        let handler_name = format!(
                            "handle_{}_{}",
                            method,
                            path.replace('/', "_")
                                .replace("{petId}", "petid")
                                .trim_matches('_')
                        );

                        Route {
                            path: path.clone(),
                            method: method.to_string().to_uppercase(),
                            handler_name,
                            schema: super::SchemaRef { name: schema_name },
                            parameters: operation
                                .parameters
                                .iter()
                                .filter_map(|param_ref| match param_ref {
                                    ReferenceOr::Item(param) => Some(RouteParameter {
                                        name: escape_rust_keyword(&param.parameter_data_ref().name),
                                        param_type: match &param.parameter_data_ref().format {
                                            ParameterSchemaOrContent::Schema(schema_ref) => {
                                                match schema_ref {
                                                    ReferenceOr::Item(schema) => {
                                                        format!("{:?}", schema.schema_kind.clone())
                                                    }
                                                    _ => "String".to_string(),
                                                }
                                            }
                                            ParameterSchemaOrContent::Content(_) => {
                                                "Content".to_string()
                                            }
                                        },
                                        required: param.parameter_data_ref().required,
                                    }),
                                    _ => None,
                                })
                                .collect(),
                            path_parameters,
                            responses: operation
                                .responses
                                .responses
                                .iter()
                                .map(|(status_code, response)| RouteResponse {
                                    status_code: status_code.to_string(),
                                    description: match response {
                                        ReferenceOr::Item(resp) => resp.description.clone(),
                                        _ => String::new(),
                                    },
                                    content_type: match response {
                                        ReferenceOr::Item(resp) => {
                                            resp.content.keys().next().cloned().unwrap_or_default()
                                        }
                                        _ => String::new(),
                                    },
                                })
                                .collect(),
                            tags: {
                                let mut tags = operation.tags.clone();
                                tags.dedup();
                                tags
                            },
                        }
                    })
                    .collect::<Vec<_>>(),
                _ => Vec::new(),
            })
            .collect()
    }
}
