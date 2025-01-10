use heck::ToSnakeCase;
use openapiv3::{OpenAPI, ReferenceOr, Schema, StatusCode};
use serde::Serialize;

/// Describes a single function signature to be generated.
#[derive(Debug, Clone, Serialize)]
pub struct FunctionSignature {
    /// The doc comments that will appear above the function definition.
    /// Example:
    /// "/// List all Todo items\n///\n/// List all Todo items from in-memory storage."
    pub doc_comment: Option<String>,

    /// The function name (e.g. "list_todos").
    pub fn_name: String,

    /// Whether the function is `async`.
    pub is_async: bool,

    /// The HTTP method (e.g. "GET", "POST", "PUT", etc.).
    pub http_method: String,

    /// The path/endpoint (e.g. "/todos").
    pub path: String,

    /// An optional 'tag' or category for grouping endpoints (e.g. "Todo").
    pub tag: String,

    /// A short summary or description of what the endpoint does.
    /// This can help populate doc comments or openapi "summary" fields.
    pub summary: Option<String>,

    /// The set of parameters expected by the function (path, query, etc.).
    pub params: Vec<ParameterSignature>,

    /// The expected request body, if any (e.g. JSON of a certain type).
    pub request_body: Option<RequestBodySignature>,

    /// The possible responses returned by this function.
    /// In OpenAPI, there can be multiple response codes (200, 400, 404, etc.).
    pub responses: Vec<ResponseSignature>,

    /// The Rust return type of the function (e.g. "Json<Vec<Todo>>").
    pub return_type: Option<String>,

    pub folder: String,
}

/// Describes a parameter that appears in a function signature (path, query, etc.).
#[derive(Debug, Clone, Serialize)]
pub struct ParameterSignature {
    /// The parameter name (e.g. "todo_id").
    pub name: String,

    /// The Rust type of this parameter (e.g. "i32", "String", etc.).
    pub rust_type: String,

    /// Where the parameter is located (path, query, header, etc.).
    pub location: ParameterLocation,

    /// Description of what this parameter does, used in doc comments and OpenAPI.
    pub description: Option<String>,
}

/// Indicates whether a parameter is found in a path, query, header, etc.
#[derive(Debug, Clone, Serialize)]
pub enum ParameterLocation {
    Path,
    Query,
    Header,
}

#[derive(Debug, Clone, Serialize)]
pub struct RequestBodySignature {
    /// The Rust type that represents the request body (e.g. "CreateTodo").
    pub rust_type: String,

    /// A brief description of the body for doc comments/OpenAPI.
    pub description: Option<String>,
}

/// Describes one possible response from the function (status code, body, etc.).
#[derive(Debug, Clone, Serialize)]
pub struct ResponseSignature {
    /// The HTTP status code (e.g. 200, 404, etc.).
    pub status: u16,

    /// Description of the response.
    pub description: Option<String>,

    /// The Rust type returned for this response (e.g. "[Todo]" or "ErrorMessage").
    pub rust_type: Option<String>,
}

impl FunctionSignature {
    /// Converts a string to snake_case format
    /// Examples:
    /// "HelloWorld" -> "hello_world"
    /// "getUserInfo" -> "get_user_info"
    /// "SomeHTTPRequest" -> "some_http_request"
    /// "get_api_v4_network_validators_validatorsByClusterHash_clusterHash" -> "get_network_validators_validators_by_cluster_hash"
    fn to_snake_case(input: &str) -> String {
        let mut result = input.to_snake_case();

        // Remove any "api_v4_" prefix
        if result.contains("api_v4_") {
            result = result.replace("api_v4_", "")
        }

        // Remove any "v4_" prefix
        if result.contains("v4") {
            result = result.replace("v4", "")
        }

        // Remove any "api"
        // if result.contains("api") {
        result = result.replace("api", "");
        // }

        // if there is a double underscore, remove one
        if result.contains("__") {
            result = result.replace("__", "_")
        }

        // Remove any underscores at the start or end of the string
        result
            .starts_with("_")
            .then(|| result = result[1..].to_string());
        result
            .ends_with("_")
            .then(|| result = result[..result.len() - 1].to_string());

        //replace any "post" prefix with "create"
        result
            .starts_with("post_")
            .then(|| result = result.replace("post_", "create_"));

        // if starts with "search_" and ends with "_search" remove the "_search" part
        if result.starts_with("search_") && result.ends_with("_search") {
            result = result.replace("_search", "")
        }

        result = result.replace("get_get", "get");

        result
    }

    pub fn new() -> Self {
        Self {
            doc_comment: None,
            fn_name: String::new(),
            is_async: true,
            http_method: String::new(),
            path: String::new(),
            tag: String::new(),
            summary: None,
            params: Vec::new(),
            request_body: None,
            responses: Vec::new(),
            return_type: None,
            folder: "default".to_string(),
        }
    }

    pub fn translate(&self, openapi: &OpenAPI) -> Vec<FunctionSignature> {
        let mut function_signatures = Vec::new();

        if openapi.paths.paths.is_empty() {
            // Create a default function signature for empty specs
            let mut func_sig = FunctionSignature::new();
            func_sig.fn_name = "default_handler".to_string();
            func_sig.path = "/".to_string();
            func_sig.http_method = "GET".to_string();
            func_sig.is_async = true;
            func_sig.doc_comment = Some("/// Default handler for empty OpenAPI spec".to_string());
            function_signatures.push(func_sig);
            return function_signatures;
        }

        for (path, path_item) in openapi.paths.iter() {
            if let ReferenceOr::Item(path_item) = path_item {
                let operations = [
                    ("GET", &path_item.get),
                    ("POST", &path_item.post),
                    ("PUT", &path_item.put),
                    ("PATCH", &path_item.patch),
                    ("DELETE", &path_item.delete),
                ];

                // self.folder = path_item.

                for (method, operation_option) in operations {
                    if let Some(operation) = operation_option {
                        let mut func_sig = FunctionSignature::new();

                        func_sig.path = path.clone();
                        func_sig.http_method = method.to_string();
                        func_sig.fn_name = operation.operation_id.clone().unwrap_or_else(|| {
                            let path_name = path.replace('/', "_").trim_matches('_').to_string();
                            if path_name.is_empty() {
                                method.to_lowercase()
                            } else {
                                format!("{}_{}", method.to_lowercase(), path_name)
                            }
                        });

                        func_sig.fn_name = FunctionSignature::to_snake_case(&func_sig.fn_name);

                        // Set folder to first non-v4 tag if available, else default
                        func_sig.folder = operation
                            .tags
                            .iter()
                            .find(|&tag| tag != "v4")
                            .map(|tag| tag.to_string())
                            .unwrap_or_else(|| "default".to_string());

                        if let Some(description) = &operation.description {
                            // Format doc comment with proper line breaks
                            let formatted_doc = description
                                .lines()
                                .map(|line| format!("/// {}", line))
                                .collect::<Vec<_>>()
                                .join("\n");
                            func_sig.doc_comment = Some(formatted_doc);
                        } else if let Some(summary) = &operation.summary {
                            // Use summary as fallback for doc comment
                            func_sig.doc_comment = Some(format!("/// {}", summary));
                        }

                        func_sig.is_async = true;

                        if !operation.tags.is_empty() {
                            func_sig.tag = operation.tags.first().unwrap().clone();
                        }

                        func_sig.summary = operation.summary.clone();

                        for param in &operation.parameters {
                            if let ReferenceOr::Item(param) = param {
                                let param_data = param.parameter_data_ref();
                                let param_sig = ParameterSignature {
                                    name: param_data.name.clone(),
                                    rust_type: match param {
                                        openapiv3::Parameter::Query { parameter_data, .. } => {
                                            match &parameter_data.format {
                                                openapiv3::ParameterSchemaOrContent::Schema(s) => {
                                                    schema_to_rust_type(s)
                                                }
                                                _ => "String".to_string(),
                                            }
                                        }
                                        openapiv3::Parameter::Path { parameter_data, .. } => {
                                            match &parameter_data.format {
                                                openapiv3::ParameterSchemaOrContent::Schema(s) => {
                                                    schema_to_rust_type(s)
                                                }
                                                _ => "String".to_string(),
                                            }
                                        }
                                        openapiv3::Parameter::Header { parameter_data, .. } => {
                                            match &parameter_data.format {
                                                openapiv3::ParameterSchemaOrContent::Schema(s) => {
                                                    schema_to_rust_type(s)
                                                }
                                                _ => "String".to_string(),
                                            }
                                        }
                                        _ => "String".to_string(),
                                    },
                                    location: match param {
                                        openapiv3::Parameter::Query { .. } => {
                                            ParameterLocation::Query
                                        }
                                        openapiv3::Parameter::Path { .. } => {
                                            ParameterLocation::Path
                                        }
                                        openapiv3::Parameter::Header { .. } => {
                                            ParameterLocation::Header
                                        }
                                        _ => ParameterLocation::Query,
                                    },
                                    description: param_data.description.clone(),
                                };
                                func_sig.params.push(param_sig);
                            }
                        }

                        if let Some(body) = &operation.request_body {
                            if let ReferenceOr::Item(body) = body {
                                if let Some(content) = body.content.get("application/json") {
                                    if let Some(schema) = &content.schema {
                                        func_sig.request_body = Some(RequestBodySignature {
                                            rust_type: schema_to_rust_type(&schema),
                                            description: body.description.clone(),
                                        });
                                    }
                                }
                            }
                        }

                        for (status_code, response) in &operation.responses.responses {
                            if let ReferenceOr::Item(response) = response {
                                if let Some(content) = response.content.get("application/json") {
                                    if let Some(schema) = &content.schema {
                                        func_sig.responses.push(ResponseSignature {
                                            status: match status_code {
                                                StatusCode::Code(code) => *code,
                                                _ => 200, // Default to 200 OK if status code is not a numeric value
                                            },
                                            description: Some(response.description.clone()),
                                            rust_type: Some(schema_to_rust_type(&schema)),
                                        });
                                    }
                                }
                            }
                        }

                        function_signatures.push(func_sig);
                    }
                }
            }
        }

        function_signatures
    }
}

fn schema_to_rust_type(schema: &ReferenceOr<Schema>) -> String {
    match schema {
        ReferenceOr::Item(s) => match &s.schema_kind {
            openapiv3::SchemaKind::Type(openapiv3::Type::String(_)) => "String".to_string(),
            openapiv3::SchemaKind::Type(openapiv3::Type::Integer(_)) => "i32".to_string(),
            openapiv3::SchemaKind::Type(openapiv3::Type::Number(_)) => "f64".to_string(),
            openapiv3::SchemaKind::Type(openapiv3::Type::Boolean(_)) => "bool".to_string(),
            openapiv3::SchemaKind::Type(openapiv3::Type::Array(array)) => {
                let item_type = match array.items.as_ref() {
                    Some(ReferenceOr::Item(s)) => {
                        schema_to_rust_type(&ReferenceOr::Item(*s.clone()))
                    }
                    _ => "serde_json::Value".to_string(),
                };
                format!("Vec<{}>", item_type)
            }
            openapiv3::SchemaKind::Type(openapiv3::Type::Object(_)) => s
                .schema_data
                .title
                .as_ref()
                .map(|t| t.clone())
                .unwrap_or_else(|| "serde_json::Value".to_string()),
            _ => "serde_json::Value".to_string(),
        },
        ReferenceOr::Reference { reference: _ } => "serde_json::Value".to_string(),
    }
}

#[cfg(test)]
#[path = "tests/functions_translator_test.rs"]
mod functions_translator_tests;
