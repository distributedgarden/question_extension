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

    // Get OpenAI API key from environment variables
    let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        error!("Warning: OPENAI_API_KEY not set. Please add it to your .env file.");
        String::new()
    });

    // Create shared application state
    let state = Arc::new(AppState {
        api_key,
        api_url: "https://api.openai.com/v1/chat/completions".to_string(),
        http_client: Client::new(),
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
    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
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

    // Call the LLM API
    match call_llm_api(&state, &prompt).await {
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

// Function to call the LLM API
async fn call_llm_api(state: &AppState, prompt: &str) -> Result<String> {
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