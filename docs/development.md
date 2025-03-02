# Question Extension Development Guide

This document provides guidance for developers who want to contribute to or modify the Question Extension project.

## Development Environment Setup

### Prerequisites

- **Rust** (1.70+): For server development
  - Install from [rustup.rs](https://rustup.rs/)
- **Node.js** (14+): For extension development utilities
  - Install from [nodejs.org](https://nodejs.org/)
- **web-ext**: Mozilla's extension packaging tool
  - Install with `npm install -g web-ext`
- **Docker** (optional): For containerized development
  - Install from [docker.com](https://www.docker.com/get-started)

### Setting Up Local Development

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/question-extension.git
   cd question-extension 
   ```

2. Server setup:
   ```bash
   cd server
   cp .env.example .env  # Create and edit with your OpenAI API key
   cargo build
   ```

3. Extension setup:
   ```bash
   cd extension
   # No build step needed for development
   ```

## Server Development

### Running in Development Mode

```bash
cd server
cargo run
```

This will run the server in development mode with auto-reloading when files change.

### Testing

```bash
cd server
cargo test
```

### Code Style

The project follows standard Rust conventions:

- Use `rustfmt` for code formatting
- Use `clippy` for linting
- Document public interfaces with rustdoc comments

To run these tools:

```bash
rustfmt src/*.rs
cargo clippy
```

### Logging

The server uses the `tracing` crate for logging. Control log levels with the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run
```

## Extension Development

### Loading the Extension in Firefox

1. Open Firefox
2. Navigate to `about:debugging`
3. Click "This Firefox" in the sidebar
4. Click "Load Temporary Add-on..."
5. Select the `manifest.json` file in the extension directory

The extension will be loaded temporarily and automatically unloaded when Firefox closes.

### Making Changes

Firefox extensions use standard web technologies:

- JavaScript for logic
- HTML for UI components
- CSS for styling

After making changes to extension files:

1. Go to `about:debugging`
2. Find the PDF LLM Assistant extension
3. Click "Reload" to apply changes

### Debugging

1. On the `about:debugging` page, click "Inspect" next to the extension
2. This opens Developer Tools connected to the extension's background script
3. Use `console.log()` statements and breakpoints for debugging

### Packaging

To create a distributable `.xpi` file:

```bash
cd extension
web-ext build
```

The packaged extension will be in the `web-ext-artifacts` directory.

## Project Structure

Understanding the project structure helps navigate the codebase:

```
question-extension/
├── .github/workflows/       # CI/CD configurations
├── extension/               # Firefox extension code
│   ├── src/                 # Source files
│   ├── manifest.json        # Extension manifest
│   └── icons/               # Extension icons
├── server/                  # Rust server code
│   ├── src/                 # Rust source code
│   ├── Cargo.toml           # Dependencies
│   └── Cargo.lock           # Locked dependencies
└── docs/                    # Documentation
```

## Adding New Features

### Adding a New Server Endpoint

1. Define the request and response models in `src/main.rs`
2. Create a handler function
3. Add the route to the router in the `main` function
4. Document the new endpoint in `docs/server.md`

Example:
```rust
// Handler for the new endpoint
async fn my_new_handler() -> impl IntoResponse {
    Json(serde_json::json!({ "message": "Hello world!" }))
}

// In the main function
let app = Router::new()
    .route("/status", get(status_handler))
    .route("/query", post(query_handler))
    .route("/new-endpoint", get(my_new_handler))  // Add your new route
    .layer(cors)
    .with_state(state);
```

### Adding New Extension Features

1. Implement the feature in the appropriate JavaScript file
2. Update the UI if needed
3. Test thoroughly across different PDFs
4. Document the new feature in `docs/extension.md`

## Cross-Platform Considerations

### Server

- Use Rust's `std::path` for cross-platform file paths
- Test on all target platforms before release
- Use GitHub Actions to build binaries for multiple platforms

### Extension

- Firefox extensions are generally cross-platform
- Test on Windows, macOS, and Linux versions of Firefox
- Be aware of PDF rendering differences between platforms

## Release Process

1. Update version numbers:
   - In `Cargo.toml` for the server
   - In `manifest.json` for the extension
   
2. Create a changelog entry

3. Merge changes to the main branch

4. Create a new GitHub release:
   - Tag with semantic versioning (e.g., `v1.0.0`)
   - Include release notes from the changelog
   - Attach built binaries and the extension package

5. For the extension, submit updated version to the Firefox Add-ons store

## Testing Strategy

### Server Testing

- Unit tests for individual functions
- Integration tests for API endpoints
- Performance benchmarks for critical paths

### Extension Testing

- Manual testing with different PDF documents
- Testing with different types of text selections
- Testing error scenarios (server down, API key invalid, etc.)

## Continuous Integration

The project uses GitHub Actions for CI/CD:

- Automatic testing on pull requests
- Building and releasing binaries on tags
- Linting and formatting checks

## Design Principles

When making changes, adhere to these principles:

1. **Security First**: Protect user data and API keys
2. **Performance**: Keep the server lightweight and responsive
3. **Simplicity**: Maintain a simple, intuitive user experience
4. **Cross-Platform**: Ensure compatibility across operating systems
5. **Documentation**: Keep documentation comprehensive and up-to-date