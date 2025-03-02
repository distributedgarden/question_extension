# Question Extension 

Question Extension is a tool that allows you to highlight text in PDF documents opened in Firefox and process it with a Large Language Model to get instant analysis, explanations, or answers.

![Example Screenshot](docs/images/screenshot.png)

## Features

- Highlight any text in PDF documents opened in Firefox
- Process the text with an LLM with a single click
- View formatted responses with Markdown support
- Copy results to clipboard
- Cross-platform support (Firefox on Windows, macOS, Linux)

## Architecture

The project consists of two main components:

1. **Firefox Extension**: The front-end that integrates with Firefox to capture highlighted text and display results
2. **Rust Server**: A high-performance backend server that communicates with the LLM API

## Quick Start

### Running the Server

You can either use the pre-built binaries or run the server from source:

#### Using Pre-built Binaries

1. Download the latest server binary for your platform from the [Releases](https://github.com/yourusername/question-extension/releases) page
2. Create a `.env` file in the same directory as the binary with your OpenAI API key:
   ```
   OPENAI_API_KEY=your_api_key_here
   ```
3. Run the binary:
   - Linux/macOS: `chmod +x question_extension && ./question_extension`
   - Windows: Double-click `question_extension.exe` or run from command line

#### Using Docker

```bash
cd server
docker-compose up -d
```

#### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/question-extension.git
cd question-extension/server

# Create .env file with your OpenAI API key
cp .env.example .env
# Edit .env to add your API key

# Build and run
cargo build --release
./target/release/question_extension
```

### Installing the Firefox Extension

1. [Install from Firefox Add-ons](https://addons.mozilla.org/en-US/firefox/addon/question-extension/) (recommended)

OR

2. Load temporarily for development:
   - Open Firefox and navigate to `about:debugging`
   - Click "This Firefox" in the sidebar
   - Click "Load Temporary Add-on..."
   - Select the `manifest.json` file in the `extension` directory

### Using the Tool

1. Open any PDF document in Firefox
2. Highlight text in the PDF
3. Right-click and select "Process this text with LLM"
4. View the analysis in the popup window

## Documentation

- [Server Documentation](docs/server.md)
- [Extension Documentation](docs/extension.md)
- [Development Guide](docs/development.md)

## Requirements

### Server Requirements
- OpenAI API key
- Internet connection
- 50MB RAM, minimal CPU usage

### Extension Requirements
- Firefox 78 or newer
- PDF documents opened directly in Firefox (not embedded)

## Development

### Building the Extension

```bash
cd extension
web-ext build
```

### Building the Server

```bash
cd server
cargo build --release
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Rust web framework
- [Firefox WebExtensions API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions) - Browser extension framework