use super::{Schema as LocalSchema, SchemaField};
use openapiv3::{OpenAPI, ReferenceOr, Schema as OpenApiSchema, SchemaKind, Type};

pub struct SchemasTranslator;

impl SchemasTranslator {
    pub fn new() -> Self {
        Self
    }

    pub fn translate(&self, openapi: &OpenAPI) -> Vec<LocalSchema> {
        openapi
            .components
            .as_ref()
            .map_or(Vec::new(), |components| {
                components
                    .schemas
                    .iter()
                    .filter_map(|(name, schema_ref)| {
                        let schema = match schema_ref {
                            ReferenceOr::Item(schema) => schema,
                            _ => return None,
                        };

                        let fields = match &schema.schema_kind {
                            SchemaKind::Type(Type::Object(obj)) => obj
                                .properties
                                .iter()
                                .filter_map(|(field_name, field_schema)| {
                                    let rust_type = if obj.required.contains(field_name) {
                                        Self::schema_to_rust_type(field_schema)?
                                    } else {
                                        format!(
                                            "Option<{}>",
                                            Self::schema_to_rust_type(field_schema)?
                                        )
                                    };
                                    Some(SchemaField {
                                        name: field_name.clone(),
                                        field_type: Self::schema_to_string(field_schema)?,
                                        rust_type,
                                        required: obj.required.contains(field_name),
                                    })
                                })
                                .collect(),
                            _ => Vec::new(),
                        };

                        // Check if this schema is used in path parameters
                        let is_path = name.to_lowercase().contains("path");
                        let path = if is_path {
                            // Generate path pattern based on schema fields
                            let path_segments = fields
                                .iter()
                                .map(|f| format!("{{{}}}", f.name))
                                .collect::<Vec<_>>()
                                .join("/");
                            Some(format!("/{}", path_segments))
                        } else {
                            None
                        };

                        // Generate default path if none was specified
                        let final_path = path.unwrap_or_else(|| {
                            format!("/{}", name.to_lowercase())
                        });

                        Some(LocalSchema {
                            name: name.clone(),
                            fields,
                            path: final_path,
                        })
                    })
                    .collect()
            })
    }

    fn schema_to_string(schema: &ReferenceOr<Box<OpenApiSchema>>) -> Option<String> {
        Self::schema_to_rust_type(schema)
    }

    fn schema_to_rust_type(schema: &ReferenceOr<Box<OpenApiSchema>>) -> Option<String> {
        match schema {
            ReferenceOr::Item(schema) => match &schema.schema_kind {
                SchemaKind::Type(typ) => match typ {
                    Type::String(_) => Some("String".to_string()),
                    Type::Number(_) => Some("f64".to_string()),
                    Type::Integer(_) => Some("i64".to_string()),
                    Type::Boolean(_) => Some("bool".to_string()),
                    Type::Array(arr) => arr.items.as_ref().and_then(|items| {
                        Self::schema_to_rust_type(items)
                            .map(|item_type| format!("Vec<{}>", item_type))
                    }),
                    Type::Object(_) => Some("HashMap<String, Value>".to_string()),
                },
                SchemaKind::OneOf { one_of } => {
                    let types = one_of.iter()
                        .filter_map(|s| match s {
                            ReferenceOr::Item(schema) => Self::schema_to_rust_type(&ReferenceOr::Item(Box::new(schema.clone()))),
                            ReferenceOr::Reference { reference } => Self::schema_to_rust_type(&ReferenceOr::Reference { reference: reference.clone() })
                        })
                        .collect::<Vec<_>>()
                        .join(" | ");
                    Some(types)
                }
                SchemaKind::AllOf { all_of } => {
                    let types = all_of.iter()
                        .filter_map(|s| match s {
                            ReferenceOr::Item(schema) => Self::schema_to_rust_type(&ReferenceOr::Item(Box::new(schema.clone()))),
                            ReferenceOr::Reference { reference } => Self::schema_to_rust_type(&ReferenceOr::Reference { reference: reference.clone() })
                        })
                        .collect::<Vec<_>>()
                        .join(" & ");
                    Some(types)
                }
                _ => Some("Value".to_string()),
            },
            ReferenceOr::Reference { reference } => {
                // Convert OpenAPI reference to Rust type name
                let type_name = reference
                    .rsplit('/')
                    .next()
                    .unwrap_or("Unknown")
                    .to_string();
                Some(Self::to_pascal_case(&type_name))
            },
        }
    }

    fn to_pascal_case(s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;

        for c in s.chars() {
            if c == '_' || c == '-' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }

        result
    }
}
