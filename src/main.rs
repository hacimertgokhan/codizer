use std::{env, fs, io};
use std::io::Write;
use std::path::Path;
use clap::{Command, Arg};
use dotenv::dotenv;
use regex::Regex;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug)]
struct FunctionDoc {
    title: String,
    description: String,
    developed_by: String,
}

#[derive(Serialize, Deserialize)]
struct AIAnalysisRequest {
    text: String,
    analysis_type: String,
    enhancement_options: EnhancementOptions,
}

#[derive(Serialize, Deserialize)]
struct EnhancementOptions {
    depth_level: String,
    target_audience: String,
    include_technical_details: bool,
    include_examples: bool,
    include_best_practices: bool,
    generate_suggestions: bool,
    output_format: String,
}

#[derive(Serialize, Deserialize)]
struct AIStudioResponse {
    candidates: Vec<Candidate>,
    usageMetadata: Option<UsageMetadata>,
}

#[derive(Serialize, Deserialize)]
struct Candidate {
    content: Content,
    finishReason: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize, Deserialize)]
struct Part {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct UsageMetadata {
    promptTokenCount: Option<u32>,
    candidatesTokenCount: Option<u32>,
    totalTokenCount: Option<u32>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let matches = Command::new("Codizer")
        .version("1.0")
        .author("Hacı Mert Gökhan")
        .about("Analyze codizer tags and generate documentation")
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
            let markdown_content = parse_codizer_tags(&contents).await;
            let output_path = &(file_name.clone() + "_documentation.md");
            match write_to_file(output_path, &markdown_content) {
                Ok(_) => println!("Documentation file created: {}", output_path),
                Err(e) => eprintln!("Error writing file: {}", e),
            }
        } else {
            eprintln!("Could not read file: {}", file_name);
        }
    } else {
        eprintln!("Please specify an input file.");
    }
}

async fn parse_codizer_tags(contents: &str) -> String {
    let re = Regex::new(r"//codizer\((.*?)\)").unwrap();
    let mut markdown_content = String::new();

    for cap in re.captures_iter(contents) {
        let codizer_tag = &cap[1];
        match parse_codizer_tag(codizer_tag) {
            Ok(function_doc) => {
                let enhanced_description = enhance_description_with_ai(&function_doc.description).await;
                markdown_content.push_str(&generate_markdown(&function_doc, &enhanced_description));
            },
            Err(e) => eprintln!("Tag parsing error: {}", e),
        }
    }

    markdown_content
}

fn parse_codizer_tag(tag: &str) -> Result<FunctionDoc, String> {
    let cleaned_tag = tag.split('\n')
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n");

    let re = Regex::new(r"(?sx)
        title='(?P<title>[^']*)'
        \s*,\s*
        description='(?P<description>[^']*)'
        \s*,\s*
        developed_by='(?P<developed_by>[^']*)'
    ").map_err(|e| e.to_string())?;

    let caps = re.captures(&cleaned_tag)
        .ok_or_else(|| format!("Tag did not match: {}", cleaned_tag))?;

    Ok(FunctionDoc {
        title: caps.name("title").map_or("", |m| m.as_str()).to_string(),
        description: caps.name("description").map_or("", |m| m.as_str()).to_string(),
        developed_by: caps.name("developed_by").map_or("", |m| m.as_str()).to_string(),
    })
}

async fn enhance_description_with_ai(description: &str) -> String {
    let api_key = env::var("API_KEY").unwrap_or_else(|_| {
        eprintln!("Warning: API_KEY not found in environment variables");
        String::from("")
    });

    if api_key.is_empty() {
        return description.to_string();
    }

    let api_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        api_key
    );

    let request_body = json!({
        "contents": [{
            "parts": [{"text": description}]
        }]
    });

    let client = reqwest::Client::new();

    let response = match client
        .post(&api_url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
    {
        Ok(resp) => {
            let status = resp.status();
            let body_text = resp.text().await.unwrap_or_else(|_| String::from("Error reading body"));

            if status.is_success() {
                match serde_json::from_str::<AIStudioResponse>(&body_text) {
                    Ok(ai_response) => {
                        if let Some(candidate) = ai_response.candidates.get(0) {
                            let enhanced_description = &candidate.content.parts[0].text;
                            return enhanced_description.to_string();
                        } else {
                            eprintln!("No candidates found in AI response.");
                            description.to_string()
                        }
                    },
                    Err(e) => {
                        eprintln!("Error parsing AI response: {}", e);
                        description.to_string()
                    }
                }
            } else {
                eprintln!("AI Studio API call failed: {}", status);
                description.to_string()
            }
        },
        Err(e) => {
            eprintln!("Error calling AI Studio API: {}", e);
            description.to_string()
        }
    };

    response
}

fn generate_markdown(doc: &FunctionDoc, enhanced_description: &str) -> String {
    let mut md = String::new();

    md.push_str(&format!("# {}\n\n", doc.title));
    md.push_str("## Description\n");
    md.push_str(enhanced_description);
    md.push_str("\n\n");
    md.push_str(&format!("## Developed by\n{}\n\n", doc.developed_by));
    md.push_str(&format!("*Last updated: {}*\n\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
    md.push_str("---\n\n");

    md
}

fn write_to_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
