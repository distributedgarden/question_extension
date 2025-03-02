# Using Question Extension with Local Llama Model

This guide explains how to set up and use the Question Extension with a local Llama 7B model instead of OpenAI's API.

## Requirements

For running Llama 7B locally:
- At least 8GB RAM (16GB recommended)
- A GPU with at least 6GB VRAM for reasonable performance
  - Using CPU-only mode is possible but will be slow
- 10GB+ of disk space for the model
- Docker and Docker Compose installed

## Setup Options

### Option 1: Using the Setup Script

Use the setup script to set up Ollama with the Llama model:

```bash
# Make the script executable
chmod +x scripts/setup-ollama.sh

# Run the setup script
./scripts/setup-ollama.sh
```

### Option 2: Manual Setup

1. **Install Ollama**:
   - Visit [Ollama's website](https://ollama.ai/) and download the appropriate version for your OS
   - Or pull the Docker image: `docker pull ollama/ollama:latest`

2. **Pull the Llama model**:
   ```bash
   # If using native Ollama
   ollama pull llama3
   
   # If using Docker
   docker run -it --rm -v ollama-data:/root/.ollama ollama/ollama pull llama3
   ```

## Configuration

### Environment Variables

Configure the extension to use the local model by setting these environment variables.
Set these in your `.env` file or directly in the environment:

```
LLM_PROVIDER=local
LOCAL_LLM_URL=http://localhost:11434/api/generate
LOCAL_LLM_MODEL=llama3
```

### Docker Compose

```bash
# Start both the PDF LLM Assistant and Ollama
docker-compose -f docker-compose.local-llama.yml up -d
```

This will start both the Rust server and Ollama in containers with the correct configuration.

## Troubleshooting

### Logs

Check the logs to diagnose issues:

```bash
# For the PDF LLM Assistant server
docker-compose -f docker-compose.local-llama.yml logs question-extension 

# For Ollama
docker-compose -f docker-compose.local-llama.yml logs ollama
```

## Alternative Models

You can use models other than `llama3` by changing the `LOCAL_LLM_MODEL` variable:

- `llama3:8b` - Smaller, faster version
- `llama3:70b` - Larger, higher quality (requires more GPU memory)
- `mistral` - Alternative model with good performance
- `gemma:7b` - Google's open model

First, pull the model with Ollama:
```bash
docker exec -it ollama ollama pull mistral
```

Then update your configuration to use it:
```
LOCAL_LLM_MODEL=mistral
```