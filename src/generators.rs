pub fn generate_markdown(promiser_content: &str) -> String {
    let method = extract_method(promiser_content);
    let format = extract_format(promiser_content);
    let body_params = extract_body_params(promiser_content);
    let markdown = format!(
        "# Endpoint Documentation\n\n**Method**: {}\n**Format**: {}\n\n**Request Body**:\n{}\n",
        method,
        format,
        body_params
    );

    markdown
}

fn extract_method(content: &str) -> String {
    if content.contains("POST") {
        "POST".to_string()
    } else if content.contains("GET") {
        "GET".to_string()
    } else {
        "Unknown".to_string()
    }
}

fn extract_format(content: &str) -> String {
    if content.contains("raw-json") {
        "raw-json".to_string()
    } else {
        "Unknown".to_string()
    }
}

fn extract_body_params(content: &str) -> String {
    let body_pattern = r"\[(.*?)\]";
    let re = regex::Regex::new(body_pattern).unwrap();
    if let Some(capture) = re.captures(content) {
        capture[1].to_string()
    } else {
        "No body parameters found".to_string()
    }
}
