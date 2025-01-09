use openapiv3::OpenAPI;

/// Describes a single function signature to be generated.
#[derive(Debug, Clone)]
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
    pub tag: Option<String>,

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
}

/// Describes a parameter that appears in a function signature (path, query, etc.).
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub enum ParameterLocation {
    Path,
    Query,
    Header,
    // You can expand further if needed
}

/// Describes the request body expected by this function (if any).
#[derive(Debug, Clone)]
pub struct RequestBodySignature {
    /// The Rust type that represents the request body (e.g. "CreateTodo").
    pub rust_type: String,

    /// A brief description of the body for doc comments/OpenAPI.
    pub description: Option<String>,
}

/// Describes one possible response from the function (status code, body, etc.).
#[derive(Debug, Clone)]
pub struct ResponseSignature {
    /// The HTTP status code (e.g. 200, 404, etc.).
    pub status: u16,

    /// Description of the response.
    pub description: Option<String>,

    /// The Rust type returned for this response (e.g. "[Todo]" or "ErrorMessage").
    pub rust_type: Option<String>,
}

impl FunctionSignature {
    /// Creates a new function signature with the given name and path.
    pub fn new() -> Self {
        Self {
            doc_comment: None,
            fn_name: String::new(),
            is_async: false,
            http_method: "GET".to_string(),
            path: String::new(),
            tag: None,
            summary: None,
            params: Vec::new(),
            request_body: None,
            responses: Vec::new(),
            return_type: None,
        }
    }
    pub fn translate(&self, openapi: &OpenAPI) -> Vec<FunctionSignature> {
        let routes_translator = FunctionSignature::new();

        vec![routes_translator]
    }
}
