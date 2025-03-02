# Using Question Extension with Local Llama Model

This guide explains how to set up and use the Question Extension with a local Llama 7B model instead of OpenAI's API.

## Overview

The Question Extension can use different LLM providers:
1. **OpenAI API** (default): Requires an internet connection and API key
2. **Local Llama model**: Runs completely locally using [Ollama](https://ollama.ai/) to serve Llama 7B

Using a local model provides:
- **Privacy**: Your data never leaves your machine
- **No API costs**: Use the assistant without paying for API calls
- **Offline usage**: Works without an internet connection (after initial setup)

## Requirements

For running Llama 7B locally:
- At least 8GB RAM (16GB recommended)
- A GPU with at least 6GB VRAM for reasonable performance
  - Using CPU-only mode is possible but will be slow
- 10GB+ of disk space for the model
- Docker and Docker Compose installed

## Setup Options

### Option 1: Using the Setup Script

We provide a script to help you set up Ollama with the Llama model:

```bash
# Make the script executable
chmod +x scripts/setup-ollama.sh

# Run the setup script
./scripts/setup-ollama.sh
```

### Option 2: Manual Setup

If you prefer to set up manually or the script doesn't work for your environment:

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

Configure the PDF LLM Assistant to use the local model by setting these environment variables:

```
LLM_PROVIDER=local
LOCAL_LLM_URL=http://localhost:11434/api/generate
LOCAL_LLM_MODEL=llama3
```

You can set these in your `.env` file or directly in the environment.

### Docker Compose

We provide a separate Docker Compose file for running with the local Llama model:

```bash
# Start both the PDF LLM Assistant and Ollama
docker-compose -f docker-compose.local-llama.yml up -d
```

This will start both the Rust server and Ollama in containers with the correct configuration.

## Usage

Once configured, the PDF LLM Assistant will use the local Llama model automatically. There's no change to how you use the Firefox extension:

1. Highlight text in a PDF
2. Right-click and select "Process this text with LLM"
3. View the results in the popup

The first query might take longer as the model loads into memory.

## Performance Considerations

- **First Query**: The first query after starting Ollama will be slow (10-30 seconds) as the model loads into GPU memory
- **Subsequent Queries**: Much faster (1-5 seconds) once the model is loaded
- **Quality**: The Llama 7B model provides good results but may not match the quality of GPT-4o
- **Memory Usage**: Monitor your system's RAM and GPU memory usage

## Troubleshooting

### Common Issues

1. **"Failed to connect to Ollama server"**:
   - Ensure Ollama is running: `docker ps | grep ollama`
   - Check Ollama logs: `docker logs ollama`

2. **"Model not found"**:
   - The model wasn't downloaded properly. Run: `docker exec -it ollama ollama pull llama3`

3. **"Out of memory"**:
   - Your system doesn't have enough RAM or VRAM
   - Try a smaller model like `llama3:8b` or `tinyllama`

4. **Slow responses**:
   - This is normal for CPU-only operation
   - Consider using a GPU if available

### Logs

Check the logs to diagnose issues:

```bash
# For the PDF LLM Assistant server
docker-compose -f docker-compose.local-llama.yml logs pdf-llm-assistant

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