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

struct AppState {
    api_key: String,
    api_url: String,
    http_client: Client,
    llm_provider: LlmProvider,
    local_llm_url: String,
}

enum LlmProvider {
    OpenAi,
    LocalLlama,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // load environment variables from .env file
    if let Err(e) = dotenv() {
        error!("Failed to load .env file: {}", e);
    }

    // LLM configuration from environment variables
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

    let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        if matches!(llm_provider, LlmProvider::OpenAi) {
            error!("Warning: OPENAI_API_KEY not set but using OpenAI provider. Please add it to your .env file.");
        }
        String::new()
    });

    let local_llm_url = env::var("LOCAL_LLM_URL").unwrap_or_else(|_| {
        if matches!(llm_provider, LlmProvider::LocalLlama) {
            info!("LOCAL_LLM_URL not set, using default (http://localhost:11434/api/generate)");
        }
        "http://localhost:11434/api/generate".to_string()
    });

    let state = Arc::new(AppState {
        api_key,
        api_url: "https://api.openai.com/v1/chat/completions".to_string(),
        http_client: Client::new(),
        llm_provider,
        local_llm_url,
    });

    // CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/status", get(status_handler))
        .route("/query", post(query_handler))
        .layer(cors)
        .with_state(state);

    // run the server
    // let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    info!("Starting server on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn status_handler() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

#[derive(Deserialize)]
struct QueryRequest {
    text: String,
}

#[derive(Serialize)]
struct ApiResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

async fn query_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<QueryRequest>,
) -> impl IntoResponse {
    let prompt = format!("Questions:\n{}", payload.text);

    let result = match state.llm_provider {
        LlmProvider::OpenAi => call_openai_api(&state, &prompt).await,
        LlmProvider::LocalLlama => call_local_llama_api(&state, &prompt).await,
    };

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

#[derive(Deserialize)]
struct LocalLlamaResponse {
    #[allow(dead_code)]
    model: String,
    response: String,
}

async fn call_openai_api(state: &AppState, prompt: &str) -> Result<String> {
    let system_content = r#"
    You are a helpful assistant that answers questions about highlighted text from PDFs and websites. 
    Be concise and informative.
    Prefer clarity over verbosity.
    Do not use LaTeX formatting.
    If there are no explicit questions, consider what questions could be asked and answer them.
    "#;

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

    let response = state
        .http_client
        .post(&state.api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", state.api_key))
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        error!("API error: {}", error_text);
        return Err(anyhow::anyhow!("API error: {}", error_text));
    }

    let response_body: OpenAIResponse = response.json().await?;
    
    if let Some(choice) = response_body.choices.first() {
        Ok(choice.message.content.clone())
    } else {
        Err(anyhow::anyhow!("Invalid response from LLM API"))
    }
}

async fn call_local_llama_api(state: &AppState, prompt: &str) -> Result<String> {
    let complete_prompt = format!(
        "You are a helpful assistant that answers questions about highlighted text from PDFs and websites. 
Be concise and informative. Prefer clarity over verbosity. Do not use LaTeX formatting.
If there are no explicit questions, consider what questions could be asked and answer them.

{}", prompt);

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
    
    let response = state
        .http_client
        .post(&state.local_llm_url)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        error!("Local LLM API error: {}", error_text);
        return Err(anyhow::anyhow!("Local LLM API error: {}", error_text));
    }

    let response_body: LocalLlamaResponse = response.json().await?;
    
    Ok(response_body.response)
}