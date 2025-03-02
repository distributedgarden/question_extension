use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dotenvy::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info, Level};

// Application state shared across handlers
struct AppState {
    api_key: String,
    api_url: String,
    http_client: Client,
    llm_provider: LlmProvider,
    local_llm_url: String,
}

// Enum to determine which LLM provider to use
enum LlmProvider {
    OpenAi,
    LocalLlama,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Load environment variables from .env file
    if let Err(e) = dotenv() {
        error!("Failed to load .env file: {}", e);
        // Continue anyway, environment variables might be set another way
    }

    // Get LLM configuration from environment variables
    let llm_provider = match env::var("LLM_PROVIDER").unwrap_or_else(|_| "openai".to_string()).as_str() {
        "local" | "llama" => {
            info!("Using local Llama model");
            LlmProvider::LocalLlama
        },
        _ => {
            info!("Using OpenAI API");
            LlmProvider::OpenAi
        },
    };

    // Get OpenAI API key from environment variables (only needed for OpenAI)
    let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        if matches!(llm_provider, LlmProvider::OpenAi) {
            error!("Warning: OPENAI_API_KEY not set but using OpenAI provider. Please add it to your .env file.");
        }
        String::new()
    });

    // Get local LLM URL
    let local_llm_url = env::var("LOCAL_LLM_URL").unwrap_or_else(|_| {
        if matches!(llm_provider, LlmProvider::LocalLlama) {
            info!("LOCAL_LLM_URL not set, using default (http://localhost:11434/api/generate)");
        }
        "http://localhost:11434/api/generate".to_string()
    });

    // Create shared application state
    let state = Arc::new(AppState {
        api_key,
        api_url: "https://api.openai.com/v1/chat/completions".to_string(),
        http_client: Client::new(),
        llm_provider,
        local_llm_url,
    });

    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with routes
    let app = Router::new()
        .route("/status", get(status_handler))
        .route("/query", post(query_handler))
        .layer(cors)
        .with_state(state);

    // Run the server
    // let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    info!("Starting server on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// Handler for the status endpoint
async fn status_handler() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

// Request model for the query endpoint
#[derive(Deserialize)]
struct QueryRequest {
    text: String,
}

// Response models
#[derive(Serialize)]
struct ApiResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

// Handler for the query endpoint
async fn query_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<QueryRequest>,
) -> impl IntoResponse {
    // Format the prompt
    let prompt = format!("Questions:\n{}", payload.text);

    // Call the appropriate LLM API based on the provider
    let result = match state.llm_provider {
        LlmProvider::OpenAi => call_openai_api(&state, &prompt).await,
        LlmProvider::LocalLlama => call_local_llama_api(&state, &prompt).await,
    };

    // Process the result
    match result {
        Ok(response) => (
            StatusCode::OK, 
            Json(ApiResponse { 
                response: Some(response), 
                error: None 
            })
        ),
        Err(e) => {
            let error_msg = format!("Error calling LLM API: {}", e);
            error!("{}", error_msg);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    response: None,
                    error: Some(error_msg),
                }),
            )
        }
    }
}

// OpenAI API request models
#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

// OpenAI API response models
#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

// Ollama/Local LLM request model
#[derive(Serialize)]
struct LocalLlamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: Option<LocalLlamaOptions>,
}

#[derive(Serialize)]
struct LocalLlamaOptions {
    temperature: f32,
    num_predict: i32,
}

// Ollama/Local LLM response model
#[derive(Deserialize)]
struct LocalLlamaResponse {
    #[allow(dead_code)] // Explicitly allow this field to be unused
    model: String,
    response: String,
}

// Function to call the OpenAI API
async fn call_openai_api(state: &AppState, prompt: &str) -> Result<String> {
    // Define the system prompt
    let system_content = r#"
    You are a helpful assistant that answers questions about highlighted text from PDFs and websites. 
    Be concise and informative.
    Prefer clarity over verbosity.
    Do not use LaTeX formatting.
    If there are no explicit questions, consider what questions could be asked and answer them.
    "#;

    // Build the request payload
    let request = OpenAIRequest {
        model: "gpt-4o".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_content.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ],
        temperature: 0.7,
        max_tokens: 500,
    };

    // Send the request to OpenAI
    let response = state
        .http_client
        .post(&state.api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", state.api_key))
        .json(&request)
        .send()
        .await?;

    // Check if the request was successful
    if !response.status().is_success() {
        let error_text = response.text().await?;
        error!("API error: {}", error_text);
        return Err(anyhow::anyhow!("API error: {}", error_text));
    }

    // Parse the response
    let response_body: OpenAIResponse = response.json().await?;
    
    // Extract the response content
    if let Some(choice) = response_body.choices.first() {
        Ok(choice.message.content.clone())
    } else {
        Err(anyhow::anyhow!("Invalid response from LLM API"))
    }
}

// Function to call local Llama model via Ollama or similar server
async fn call_local_llama_api(state: &AppState, prompt: &str) -> Result<String> {
    // Create a complete prompt with system instructions
    let complete_prompt = format!(
        "You are a helpful assistant that answers questions about highlighted text from PDFs and websites. 
Be concise and informative. Prefer clarity over verbosity. Do not use LaTeX formatting.
If there are no explicit questions, consider what questions could be asked and answer them.

{}", prompt);

    // Build the request payload
    let request = LocalLlamaRequest {
        model: env::var("LOCAL_LLM_MODEL").unwrap_or_else(|_| "llama3".to_string()),
        prompt: complete_prompt,
        stream: false,
        options: Some(LocalLlamaOptions {
            temperature: 0.7,
            num_predict: 500,
        }),
    };

    info!("Calling local LLM at: {}", state.local_llm_url);
    
    // Send the request to the local LLM server
    let response = state
        .http_client
        .post(&state.local_llm_url)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    // Check if the request was successful
    if !response.status().is_success() {
        let error_text = response.text().await?;
        error!("Local LLM API error: {}", error_text);
        return Err(anyhow::anyhow!("Local LLM API error: {}", error_text));
    }

    // Parse the response
    let response_body: LocalLlamaResponse = response.json().await?;
    
    // Return the response text
    Ok(response_body.response)
}