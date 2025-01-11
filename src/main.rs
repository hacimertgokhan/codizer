use std::{fs, io};
use std::io::Write;
use std::path::Path;
use clap::{Command, Arg};
use regex::Regex;

pub mod commands;
pub mod parsers;
pub mod generators;

fn main() {
    let matches = Command::new("Promizer")
        .version("1.0")
        .author("Hacı Mert Gökhan")
        .about("Analyze promiser tags and generate Markdown documentation")
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
            let output_path = "api_documentation.md";
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
        let (method, format, body) = parse_promiser_tag(promiser_tag);
        markdown_content.push_str(&format!(
            "# Endpoint Documentation\n\n**Type**: {}\n**Format**: {}\n\n**Request Body**:\n{}\n\n",
            method, format, body
        ));
    }

    markdown_content
}

fn parse_promiser_tag(tag: &str) -> (String, String, String) {
    let mut method = String::new();
    let mut format = String::new();
    let mut body = String::new();

    let re = Regex::new(r"type='(.*?)'.*format='(.*?)'.*body=\[(.*?)\]").unwrap();
    if let Some(caps) = re.captures(tag) {
        method = caps[1].to_string();
        format = caps[2].to_string();
        body = caps[3].replace("'", "");
    }

    (method, format, body)
}

fn write_to_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
