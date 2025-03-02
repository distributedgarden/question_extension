# Question Extension Server Documentation

This document provides detailed information about the Rust-based server component of the Question Extension server.

## Overview

The Question Extension server is a Rust application that handles requests from the Firefox extension, processes text using an LLM (Large Language Model), and returns the results. It serves as an intermediary between the browser extension and the LLM API (e.g., OpenAI).

## API Endpoints

### GET /status

Checks if the server is running.

**Request:**
```
GET /status HTTP/1.1
Host: localhost:5000
```

**Response:**
```json
{
  "status": "ok"
}
```

### POST /query

Processes highlighted text with the LLM.

**Request:**
```
POST /query HTTP/1.1
Host: localhost:5000
Content-Type: application/json

{
  "text": "Your highlighted text here"
}
```

**Response (Success):**
```json
{
  "response": "LLM's analysis of the text"
}
```

**Response (Error):**
```json
{
  "error": "Error message here"
}
```

## Configuration

The server is configured using environment variables:

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `OPENAI_API_KEY` | API key for OpenAI | Yes | None |

## System Requirements

- CPU: Any modern CPU (2+ cores recommended)
- RAM: 50MB minimum
- Disk: 20MB for the binary
- Network: Internet connection required for API access

## Architecture

The server is built using:

- **Axum**: Web framework for handling HTTP requests
- **Tokio**: Asynchronous runtime
- **Reqwest**: HTTP client for making API calls
- **Tracing**: Logging infrastructure

### Request Flow

1. The extension sends a request to the server
2. The server validates the request
3. The server formats a prompt for the LLM
4. The server sends the prompt to the OpenAI API
5. The server receives the response and forwards it to the extension

## Building from Source

Prerequisites:
- Rust 1.70 or newer
- Cargo (comes with Rust)

Steps:
```bash
# Navigate to the server directory
cd server

# Build in release mode
cargo build --release

# The binary will be in target/release/
```

Cross-Compilation Steps:
```bash
cargo install cross

# Build for Linux (x86_64)
cross build --release --target x86_64-unknown-linux-musl

# Build for macOS
cross build --release --target x86_64-apple-darwin

# Build for Windows
cross build --release --target x86_64-pc-windows-gnu
```

## Deployment Options

### Standalone Binary

The simplest deployment method is to run the compiled binary directly:

```bash
./question_extension
```

Ensure the `.env` file with your OpenAI API key is in the same directory.

### Docker

You can also deploy using Docker:

```bash
# Build the Docker image
docker build -t question-extension .

# Run the container
docker run -p 5000:5000 --env-file .env -d question-extension 
```

### Docker Compose

For easier management, use Docker Compose:

```bash
docker-compose up -d
```

## Logging

The server logs to stdout by default. Log levels can be controlled via the `RUST_LOG` environment variable:

```bash
# Set log level to debug
RUST_LOG=debug ./question_extension
```