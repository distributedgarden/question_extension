#!/bin/bash
# Script to set up Ollama with Llama model for Question Extension 

# Print colored messages
function print_message() {
  echo -e "\e[1;34m>> $1\e[0m"
}

# Print error messages
function print_error() {
  echo -e "\e[1;31mERROR: $1\e[0m"
}

print_message "Setting up Ollama for Question Extension"

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    print_error "Docker is not installed. Please install Docker first."
    exit 1
fi

# Check if Docker Compose is installed
if ! command -v docker-compose &> /dev/null; then
    print_error "Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi

# Pull the Ollama image
print_message "Pulling the Ollama image..."
docker pull ollama/ollama:latest

# Start Ollama container
print_message "Starting Ollama container..."
docker run -d --name ollama -p 11434:11434 -v ollama-data:/root/.ollama ollama/ollama:latest

# Wait for Ollama to start
print_message "Waiting for Ollama to start..."
sleep 10

# Pull the Llama model
print_message "Pulling the Llama model (this may take a while)..."
docker exec -it ollama ollama pull llama3

# Check if the model was pulled successfully
if [ $? -ne 0 ]; then
    print_error "Failed to pull the Llama model. Please check the Ollama logs."
    exit 1
fi

# Stop the container
print_message "Stopping the Ollama container..."
docker stop ollama
docker rm ollama

print_message "Setup complete! You can now run the Question Extension server with local Llama model."
print_message "To start the services, run:"
print_message "docker-compose -f docker-compose.local-llama.yml up -d"