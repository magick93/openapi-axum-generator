use askama::Result;

pub fn sanitize_handler_name(name: &str) -> Result<String> {
    let name = name.replace("{", "").replace("}", "");
    let name = name.replace("-", "_");
    Ok(name)
}

pub fn title(s: &str) -> Result<String> {
    let mut result = String::new();
    let mut capitalize = true;

    for c in s.chars() {
        if capitalize {
            result.push(c.to_ascii_uppercase());
            capitalize = false;
        } else {
            result.push(c);
        }

        if c == ' ' {
            capitalize = true;
        }
    }

    Ok(result)
}

pub fn replace(s: &str, old: &str, new: &str) -> Result<String> {
    Ok(s.replace(old, new))
}
