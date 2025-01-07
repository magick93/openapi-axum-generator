use askama::Result;

pub fn sanitize_handler_name(name: &str) -> Result<String> {
    let name = name.replace("{", "").replace("}", "");
    let name = name.replace("-", "_");
    Ok(name)
}
