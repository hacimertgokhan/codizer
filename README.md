# Codizer

**Codizer** is a Rust CLI tool designed to analyze custom `//codizer()` tags in source code and generate structured documentation. It also uses an AI service to enhance descriptions with technical details, best practices, and complexity assessments.

## Features

- **Tag Parsing**: Extracts information from `//codizer()` tags within the source code.
- **AI-Enhanced Descriptions**: Enriches function descriptions using AI, providing comprehensive technical details, best practices, and suggestions.
- **Documentation Generation**: Automatically creates a Markdown documentation file based on the parsed code comments.

## Requirements

- **Rust** (version 1.56 or higher)
- [dotenv crate](https://crates.io/crates/dotenv) to manage environment variables
- [clap crate](https://crates.io/crates/clap) for command-line argument parsing
- [reqwest crate](https://crates.io/crates/reqwest) for making HTTP requests
- [regex crate](https://crates.io/crates/regex) for parsing tags
- [serde crate](https://crates.io/crates/serde) for serializing and deserializing JSON
- **AI Studio API Key** for making requests to enhance documentation

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/hacimertgokhan/codizer.git
   cd codizer
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Set up your `.env` file to include your AI Studio API key:
   ```bash
   echo "API_KEY=your_api_key_here" > .env
   ```

## Usage

Codizer can be used from the command line to analyze your source files and generate enhanced documentation.

### Example Command

```bash
cargo run -- --input <your_source_file>
```

### Arguments

- `--input` or `-i`: The input source file containing `//codizer()` tags for analysis.

### Example

If you have a file `server.js` with `//codizer()` tags:

```bash
cargo run -- --input server.js
```

This will generate a Markdown file named `server.js_documentation.md` with the parsed documentation and enhanced descriptions.

## Codizer Tag Format

The `//codizer()` tag format is as follows:

```text
//codizer(title='Function Title', description='Brief description of the function', developed_by='Author Name')
```

### Example Tag

```rust
//codizer(title='Encrypt Data', description='Encrypts data using AES encryption', developed_by='John Doe')
```

## AI Integration

Codizer utilizes the AI Studio API to provide enhanced descriptions. The API response includes:

- **Enhanced Descriptions**: An AI-enriched version of the description.
- **Technical Details**: Additional insights about the function.
- **Suggestions**: AI-driven best practices or improvements.
- **Complexity Score**: A numeric complexity rating from 1 to 10.

## Error Handling

Codizer provides error messages for:

- Missing API keys in the `.env` file.
- Failed API requests.
- Tag parsing errors if the format is incorrect.
