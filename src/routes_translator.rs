use openapiv3::{OpenAPI, Operation, Parameter, ParameterData, ParameterSchemaOrContent, PathItem, ReferenceOr, Response, Schema, SchemaKind, Type};

use super::{Parameter as RouteParameter, Response as RouteResponse, Route};

pub struct RoutesTranslator;

impl RoutesTranslator {
    pub fn new() -> Self {
        Self
    }

    pub fn translate(&self, openapi: &OpenAPI) -> Vec<Route> {
        openapi.paths
            .iter()
            .flat_map(|(path, item)| match item {
                ReferenceOr::Item(path_item) => path_item
                    .iter()
                    .flat_map(|(method, operation)| std::iter::once((method, operation)))
                    .map(|(method, operation)| Route {
                        path: path.clone(),
                        method: method.to_string().to_uppercase(),
                        handler_name: format!(
                            "handle_{}_{}",
                            method,
                            path.replace('/', "_").trim_matches('_')
                        ),
                        parameters: operation
                            .parameters
                            .iter()
                            .filter_map(|param_ref| match param_ref {
                                ReferenceOr::Item(param) => Some(RouteParameter {
                                    name: param.parameter_data_ref().name.clone(),
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
                    })
                    .collect::<Vec<_>>(),
                _ => Vec::new(),
            })
            .collect()
    }
}
