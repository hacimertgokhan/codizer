use std::{fs, io};
use std::io::Write;
use std::path::Path;
use clap::{Command, Arg};
use regex::Regex;

pub mod commands;
pub mod parsers;
pub mod generators;

#[derive(Debug)]
struct EndpointDoc {
    path: String,
    base_url: String,
    method: String,
    format: String,
    body: String,
    description: String,
    parameters: Vec<Parameter>,
    responses: Vec<Response>,
    tags: Vec<String>,
    security: Vec<String>,
    consumes: Vec<String>,
    produces: Vec<String>,
    deprecated: bool,
}

#[derive(Debug)]
struct Parameter {
    name: String,
    location: String,
    required: bool,
    type_: String,
    description: String,
}

#[derive(Debug)]
struct Response {
    code: String,
    description: String,
    schema: String,
}

fn main() {
    let matches = Command::new("Promizer")
        .version("1.0")
        .author("Hacı Mert Gökhan")
        .about("Analyze promiser tags and generate Swagger-like Markdown documentation")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Input file to analyze"),
        )
        .get_matches();

    if let Some(file_name) = matches.get_one::<String>("input") {
        if let Ok(contents) = fs::read_to_string(file_name) {
            let markdown_content = parse_promiser_tags(&contents);
            let output_path = &(file_name.clone() + "_api_documentation.md");
            match write_to_file(output_path, &markdown_content) {
                Ok(_) => println!("Markdown dosyası oluşturuldu: {}", output_path),
                Err(e) => eprintln!("Dosya yazılırken hata oluştu: {}", e),
            }
        } else {
            eprintln!("Dosya okunamadı: {}", file_name);
        }
    } else {
        eprintln!("Lütfen bir dosya adı belirtin.");
    }
}

fn parse_promiser_tags(contents: &str) -> String {
    let re = Regex::new(r"//promizer\((.*?)\)").unwrap();
    let mut markdown_content = String::new();

    for cap in re.captures_iter(contents) {
        let promiser_tag = &cap[1];
        println!("Parsing tag: {}", promiser_tag); // Debug için
        match parse_promiser_tag(promiser_tag) {
            Ok(endpoint_doc) => markdown_content.push_str(&generate_markdown(&endpoint_doc)),
            Err(e) => eprintln!("Tag ayrıştırma hatası: {}", e),
        }
    }

    markdown_content
}

fn parse_promiser_tag(tag: &str) -> Result<EndpointDoc, String> {
    // Whitespace'leri temizle ama new line'ları koru
    let cleaned_tag = tag.split('\n')
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n");

    let re = Regex::new(r"(?sx)
        path='(?P<path>[^']*)'
        \s*,\s*
        url='(?P<url>[^']*)'
        \s*,\s*
        type='(?P<type>[^']*)'
        \s*,\s*
        format='(?P<format>[^']*)'
        \s*,\s*
        description='(?P<description>[^']*)'
        \s*,\s*
        parameters=\[(?P<parameters>.*?)\]
        \s*,\s*
        responses=\[(?P<responses>.*?)\]
        \s*,\s*
        tags=\[(?P<tags>.*?)\]
        \s*,\s*
        security=\[(?P<security>.*?)\]
        \s*,\s*
        consumes=\[(?P<consumes>.*?)\]
        \s*,\s*
        produces=\[(?P<produces>.*?)\]
        \s*,\s*
        deprecated='(?P<deprecated>[^']*)'
        \s*,\s*
        body=\[(?P<body>.*?)\]
    ").map_err(|e| e.to_string())?;

    let caps = re.captures(&cleaned_tag)
        .ok_or_else(|| format!("Tag eşleşmedi: {}", cleaned_tag))?;

    Ok(EndpointDoc {
        path: caps.name("path").map_or("", |m| m.as_str()).to_string(),
        base_url: caps.name("url").map_or("", |m| m.as_str()).to_string(),
        method: caps.name("type").map_or("", |m| m.as_str()).to_string(),
        format: caps.name("format").map_or("", |m| m.as_str()).to_string(),
        description: caps.name("description").map_or("", |m| m.as_str()).to_string(),
        parameters: parse_parameters_flexible(caps.name("parameters").map_or("", |m| m.as_str())),
        responses: parse_responses_flexible(caps.name("responses").map_or("", |m| m.as_str())),
        tags: parse_array(caps.name("tags").map_or("", |m| m.as_str())),
        security: parse_array(caps.name("security").map_or("", |m| m.as_str())),
        consumes: parse_array(caps.name("consumes").map_or("", |m| m.as_str())),
        produces: parse_array(caps.name("produces").map_or("", |m| m.as_str())),
        deprecated: caps.name("deprecated").map_or("false", |m| m.as_str()) == "true",
        body: clean_json_string(caps.name("body").map_or("", |m| m.as_str())),
    })
}

fn parse_parameters_flexible(params_str: &str) -> Vec<Parameter> {
    let cleaned_params = params_str.split('\n')
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n");

    let re = Regex::new(r"(?sx)
        \{
        \s*name='([^']*)'
        \s*,\s*
        location='([^']*)'
        \s*,\s*
        required='([^']*)'
        \s*,\s*
        type='([^']*)'
        \s*,\s*
        description='([^']*)'
        \s*\}
    ").unwrap();

    re.captures_iter(&cleaned_params)
        .map(|caps| Parameter {
            name: caps[1].to_string(),
            location: caps[2].to_string(),
            required: &caps[3] == "true",
            type_: caps[4].to_string(),
            description: caps[5].to_string(),
        })
        .collect()
}

fn parse_responses_flexible(responses_str: &str) -> Vec<Response> {
    let cleaned_responses = responses_str.split('\n')
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n");

    let re = Regex::new(r"(?sx)
        \{
        \s*code='([^']*)'
        \s*,\s*
        description='([^']*)'
        \s*,\s*
        schema='([^']*)'
        \s*\}
    ").unwrap();

    re.captures_iter(&cleaned_responses)
        .map(|caps| Response {
            code: caps[1].to_string(),
            description: caps[2].to_string(),
            schema: caps[3].to_string(),
        })
        .collect()
}

fn clean_json_string(json_str: &str) -> String {
    json_str.split('\n')
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n")
        .replace("'", "")
}

fn parse_array(array_str: &str) -> Vec<String> {
    array_str.split(',')
        .map(|s| s.trim().trim_matches('\'').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn generate_markdown(doc: &EndpointDoc) -> String {
    let mut md = String::new();

    md.push_str(&format!("# {}{}\n\n", doc.base_url, doc.path));
    if doc.deprecated {
        md.push_str("**DEPRECATED**\n\n");
    }

    md.push_str(&format!("**Method**: {}\n", doc.method));
    md.push_str(&format!("**Format**: {}\n", doc.format));
    md.push_str(&format!("\n**Description**: {}\n", doc.description));

    if !doc.tags.is_empty() {
        md.push_str(&format!("\n**Tags**: {}\n", doc.tags.join(", ")));
    }

    if !doc.security.is_empty() {
        md.push_str(&format!("\n**Security**: {}\n", doc.security.join(", ")));
    }

    if !doc.consumes.is_empty() {
        md.push_str(&format!("\n**Consumes**: {}\n", doc.consumes.join(", ")));
    }

    if !doc.produces.is_empty() {
        md.push_str(&format!("\n**Produces**: {}\n", doc.produces.join(", ")));
    }

    if !doc.parameters.is_empty() {
        md.push_str("\n## Parameters\n\n");
        for param in &doc.parameters {
            md.push_str(&format!(
                "- **{}** ({}, {})\n  - Type: {}\n  - Description: {}\n  - Required: {}\n\n",
                param.name, param.location, param.type_, param.type_, param.description, param.required
            ));
        }
    }

    if !doc.responses.is_empty() {
        md.push_str("\n## Responses\n\n");
        for response in &doc.responses {
            md.push_str(&format!(
                "### {}\n- Description: {}\n- Schema: {}\n\n",
                response.code, response.description, response.schema
            ));
        }
    }

    if !doc.body.is_empty() {
        md.push_str("\n## Request Body\n\n");
        md.push_str(&format!("```json\n{}\n```\n", doc.body));
    }

    md.push_str("\n---\n\n");
    md
}

fn write_to_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}