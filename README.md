# Promizer - Open API Markdown Documentation Generator

Promizer is a command-line tool designed to analyze `promiser` tags in JavaScript (or Node.js) files and generate API documentation in Markdown format. It helps developers automate the process of creating documentation for endpoints defined with special tags in the code.

## Features

- **Analyze Promiser Tags**: Scans JavaScript files for `promiser` tags and extracts relevant information about API endpoints.
- **Generate Markdown Documentation**: Converts extracted data into a structured Markdown format for API documentation.
- **Command-Line Interface (CLI)**: Simple and intuitive CLI to run the tool and generate documentation.

## Installation

### Prerequisites

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

1. Install Rust from the official website if you haven't already.
2. Clone this repository:

```bash
git clone https://github.com/hacimertgokhan/promizer.git
```

3. Navigate to the project directory:

```bash
cd promizer
```

4. Build the project using Cargo:

```bash
cargo build --release
```

## Usage

To use Promizer, run it from the command line with the `-i` or `--input` flag to specify the JavaScript file to analyze. The tool will output the generated API documentation in a Markdown file.

### Command Example

```bash
cargo run -- -i server.js
```

This will process the `server.js` file and create a `api_documentation.md` file in the current directory.

### Options

- `-i, --input FILE`: Specifies the input file (JavaScript file) to analyze for `promiser` tags.

## How It Works

1. **Find Promiser Tags**: The tool searches for `//promizer()` tags in the provided file. Each tag contains metadata about an API endpoint such as HTTP method, format, and request body.

2. **Parse the Tags**: Each tag is parsed to extract useful information like:
    - HTTP method (e.g., GET, POST)
    - Format (e.g., JSON, XML)
    - Request body parameters

3. **Generate Documentation**: The extracted data is formatted into a readable Markdown format and written to a `.md` file.

## Example Input (`server.js`)

```javascript
//promizer(type='GET', format='json', body=['user_id'])
//promizer(type='POST', format='json', body=['name', 'email'])
```

## Example Output (`api_documentation.md`)

```markdown
# Endpoint Documentation

**Type**: GET  
**Format**: json  

**Request Body**:  
- user_id

---

# Endpoint Documentation

**Type**: POST  
**Format**: json  

**Request Body**:  
- name  
- email
```

## Contributing

We welcome contributions to improve the project! Here are a few ways you can contribute:

- **Bug Reports**: If you find any bugs, please open an issue in the GitHub repository.
- **Feature Requests**: If you have any ideas for new features or improvements, feel free to create an issue or submit a pull request.
- **Documentation**: Help improve this documentation by submitting a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

For more information, you can contact the project maintainer at:  
**Email**: hacimertgokhan@gmail.com  
**GitHub**: [hacimertgokhan](https://github.com/hacimertgokhan)
```
