use openapiv3::{OpenAPI, ReferenceOr, Schema as OpenApiSchema, SchemaKind, Type};
use super::{SchemaField, Schema as LocalSchema};

pub struct SchemasTranslator;

impl SchemasTranslator {
    pub fn new() -> Self {
        Self
    }

    pub fn translate(&self, openapi: &OpenAPI) -> Vec<LocalSchema> {
        openapi.components
            .as_ref()
            .map_or(Vec::new(), |components| {
                components.schemas.iter()
                    .filter_map(|(name, schema_ref)| {
                        let schema = match schema_ref {
                            ReferenceOr::Item(schema) => schema,
                            _ => return None,
                        };

                        let fields = match &schema.schema_kind {
                            SchemaKind::Type(Type::Object(obj)) => obj.properties.iter()
                                .filter_map(|(field_name, field_schema)| {
                                    let rust_type = if obj.required.contains(field_name) {
                                        Self::schema_to_rust_type(field_schema)?
                                    } else {
                                        format!("Option<{}>", Self::schema_to_rust_type(field_schema)?)
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

                        Some(LocalSchema {
                            name: name.clone(),
                            fields,
                        })
                    })
                    .collect()
            })
    }

    fn schema_to_string(schema: &ReferenceOr<Box<OpenApiSchema>>) -> Option<String> {
        match schema {
            ReferenceOr::Item(schema) => match &schema.schema_kind {
                SchemaKind::Type(typ) => match typ {
                    Type::String(_) => Some("String".to_string()),
                    Type::Number(_) => Some("f64".to_string()),
                    Type::Integer(_) => Some("i64".to_string()),
                    Type::Boolean(_) => Some("bool".to_string()),
                    Type::Array(arr) => {
                        arr.items.as_ref().and_then(|items| {
                            Self::schema_to_string(items).map(|item_type| format!("Vec<{}>", item_type))
                        })
                    },
                    Type::Object(_) => Some("Object".to_string()),
                },
                _ => Some("Unknown".to_string()),
            },
            ReferenceOr::Reference { reference } => Some(reference.clone()),
        }
    }

    fn schema_to_rust_type(schema: &ReferenceOr<Box<OpenApiSchema>>) -> Option<String> {
        match schema {
            ReferenceOr::Item(schema) => match &schema.schema_kind {
                SchemaKind::Type(typ) => match typ {
                    Type::String(_) => Some("String".to_string()),
                    Type::Number(_) => Some("f64".to_string()),
                    Type::Integer(_) => Some("i64".to_string()),
                    Type::Boolean(_) => Some("bool".to_string()),
                    Type::Array(arr) => {
                        arr.items.as_ref().and_then(|items| {
                            Self::schema_to_rust_type(items).map(|item_type| format!("Vec<{}>", item_type))
                        })
                    },
                    Type::Object(_) => Some("Object".to_string()),
                },
                _ => Some("Unknown".to_string()),
            },
            ReferenceOr::Reference { reference } => Some(reference.clone()),
        }
    }
}
