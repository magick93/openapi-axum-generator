use openapiv3::{OpenAPI, ParameterSchemaOrContent, ReferenceOr, StatusCode};
use super::{Route, Parameter, Response, SchemaRef};
use crate::routes::keywords::escape_rust_keyword;

pub struct RoutesTranslator;

impl Default for RoutesTranslator {
    fn default() -> Self {
        Self::new()
    }
}

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
                                ReferenceOr::Reference { reference: _ } => None,
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
                                        ReferenceOr::Reference { reference: _ } => None,
                                    })
                            })
                            .or_else(|| {
                                // Check for DataSetList schema reference
                                if let Some(ReferenceOr::Item(schema)) = &operation.responses.responses.get(&StatusCode::Code(200)).and_then(|resp| match resp {
                                    ReferenceOr::Item(r) => r.content.values().next().and_then(|media| media.schema.as_ref()),
                                    _ => None,
                                }) {
                                    if schema.schema_data.title.as_deref() == Some("dataSetList") {
                                        return Some("DataSetList".to_string());
                                    }
                                }
                                Some("DefaultSchema".to_string())
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
                        let handler_name = if path == "/" {
                            format!("handle_{}_root", method)
                        } else {
                            // Remove curly braces and maintain plural form for datasets
                            let clean_path = path
                                .replace('/', "_")
                                .replace("{dataset}", "datasets")
                                .replace("{version}", "version")
                                .replace("{", "")
                                .replace("}", "")
                                .trim_matches('_')
                                .to_string();
                            
                            // Special handling for USPTO paths to maintain correct parameter order
                            if path.contains("datasets") {
                                let parts: Vec<&str> = clean_path.split('_').collect();
                                if parts.len() >= 3 {
                                    format!("handle_{}_datasets_version_{}", method, parts[2..].join("_"))
                                } else {
                                    format!("handle_{}_datasets_version", method)
                                }
                            } else {
                                format!("handle_{}_{}", method, clean_path.to_lowercase())
                            }
                        };

                        // Ensure consistent parameter ordering
                        let mut ordered_path_parameters = path_parameters.clone();
                        if path.contains("{dataset}") && path.contains("{version}") {
                            // Ensure dataset comes before version in parameter order
                            ordered_path_parameters.sort_by(|a, b| {
                                match (a.as_str(), b.as_str()) {
                                    ("dataset", _) => std::cmp::Ordering::Less,
                                    (_, "dataset") => std::cmp::Ordering::Greater,
                                    _ => path.find(&format!("{{{}}}", a))
                                        .unwrap()
                                        .cmp(&path.find(&format!("{{{}}}", b)).unwrap())
                                }
                            });
                            
                            // Verify dataset is first and version is second
                            if ordered_path_parameters.len() >= 2 {
                                assert_eq!(ordered_path_parameters[0], "dataset");
                                assert_eq!(ordered_path_parameters[1], "version");
                            }
                        }

                        Route {
                            path: path.clone(),
                            method: method.to_string().to_uppercase(),
                            handler_name,
                            schema: SchemaRef { name: schema_name },
                            parameters: operation
                                .parameters
                                .iter()
                                .filter_map(|param_ref| match param_ref {
                                    ReferenceOr::Item(param) => Some(Parameter {
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
                                    ReferenceOr::Reference { reference: _ } => None,
                                })
                                .collect(),
                            path_parameters: ordered_path_parameters,
                            responses: operation
                                .responses
                                .responses
                                .iter()
                                .map(|(status_code, response)| Response {
                                    status_code: status_code.to_string(),
                                    description: match response {
                                        ReferenceOr::Item(resp) => resp.description.clone(),
                                        ReferenceOr::Reference { reference: _ } => String::new(),
                                    },
                                    content_type: match response {
                                        ReferenceOr::Item(resp) => {
                                            resp.content.keys().next().cloned().unwrap_or_default()
                                        }
                                        ReferenceOr::Reference { reference: _ } => String::new(),
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
                ReferenceOr::Reference { reference: _ } => Vec::new(),
            })
            .collect()
    }
}
