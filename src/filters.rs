use askama::Result;

pub fn is_pet_id_route(path: &str) -> Result<String> {
    if path.contains("{petId}") {
        Ok("_petid".to_string())
    } else {
        Ok("".to_string())
    }
}

pub fn display_some(value: &str) -> askama::Result<String> {
    Ok(value.to_string())
}

pub fn snake_case(input: &str) -> Result<String> {
    Ok(input.replace("::", "_").to_lowercase())
}

pub fn sanitize_handler_name(input: &str) -> Result<String> {
    Ok(input
        .replace("::", "_")
        .replace("{", "")
        .replace("}", "")
        .to_lowercase())
}
