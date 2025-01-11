use regex::Regex;

pub fn parse_promiser(input: &str) -> Option<(String, String, Vec<String>)> {
    let re = Regex::new(r"promizer\(type='(.*?)', format='(.*?)', body=\[(.*?)\]\)").unwrap();
    if let Some(caps) = re.captures(input) {
        let method = caps[1].to_string();
        let format = caps[2].to_string();
        let body_params: Vec<String> = caps[3]
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        return Some((method, format, body_params));
    }
    None
}
