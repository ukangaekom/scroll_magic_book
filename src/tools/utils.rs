
pub fn extract_tool_params(input: &str) -> Option<(String, Vec<String>)> {
    // Find the part inside the brackets
    let start = input.find('[')?;
    let end = input.find(']')?;

    // Extract the inside and split by comma
    let content = &input[start + 1..end];
    let mut parts = content.split(',').map(|s| s.trim()).filter(|s| !s.is_empty());

    let first = parts.next()?.to_string();
    let rest = parts.map(|s| s.to_string()).collect();

    Some((first, rest))
}
