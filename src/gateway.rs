use crate::error::AppError;
use async_trait::async_trait;
use reqwest::{Client as HttpClient, Url};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

#[async_trait]
pub trait GenAiClient: Send + Sync {
    async fn chat(&self, model: &str, messages: Vec<Message>) -> Result<String, AppError>;
}

pub struct Client {
    http: HttpClient,
    base_url: Url,
    api_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Debug)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Deserialize, Debug)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

impl Client {
    pub fn new() -> Result<Self, AppError> {
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| AppError::ConfigError("OPENAI_API_KEY must be set".into()))?;

        // Allow overriding base_url for testing or other providers
        let mut base_url_str = env::var("OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1/".to_string());

        if !base_url_str.ends_with('/') {
            base_url_str.push('/');
        }

        // Ensure base_url ends with a slash if it doesn't
        let base_url = Url::parse(&base_url_str)
            .map_err(|e| AppError::ConfigError(format!("Invalid base URL: {}", e)))?;

        let http = HttpClient::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        Ok(Self {
            http,
            base_url,
            api_key,
        })
    }

    pub fn new_with_base_url(base_url_str: &str) -> Result<Self, AppError> {
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| AppError::ConfigError("OPENAI_API_KEY must be set".into()))?;

        // We assume test callers provide correct URLs, but we can also normalize here if needed.
        // For new_with_base_url it's explicitly passed so we trust it or normalization might conflict with expected test values.
        let base_url = Url::parse(base_url_str)
            .map_err(|e| AppError::ConfigError(format!("Invalid base URL: {}", e)))?;

        let http = HttpClient::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        Ok(Self {
            http,
            base_url,
            api_key,
        })
    }
}

#[async_trait]
impl GenAiClient for Client {
    async fn chat(&self, model: &str, messages: Vec<Message>) -> Result<String, AppError> {
        let url = self
            .base_url
            .join("chat/completions")
            .map_err(|e| AppError::ConfigError(format!("Failed to join URL: {}", e)))?;

        let body = ChatCompletionRequest {
            model: model.to_string(),
            messages,
        };

        let res = self
            .http
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res
                .text()
                .await
                .unwrap_or_else(|e| format!("(failed to read response body: {})", e));
            return Err(AppError::NetworkError(format!(
                "API Request failed: {} - {}",
                status, text
            )));
        }

        let response_body: ChatCompletionResponse = res
            .json()
            .await
            .map_err(|e| AppError::NetworkError(format!("Failed to parse response: {}", e)))?;

        if let Some(choice) = response_body.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(AppError::NetworkError("No choices in response".into()))
        }
    }
}

pub struct MockGenAiClient {
    pub response: String,
}

#[async_trait]
impl GenAiClient for MockGenAiClient {
    async fn chat(&self, _model: &str, _messages: Vec<Message>) -> Result<String, AppError> {
        Ok(self.response.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_chat_success() {
        let mock_server = MockServer::start().await;

        env::set_var("OPENAI_API_KEY", "test-key");

        let response_body = r#"
        {
            "choices": [
                {
                    "message": {
                        "role": "assistant",
                        "content": "Hello world"
                    }
                }
            ]
        }
        "#;

        Mock::given(method("POST"))
            .and(path("/chat/completions"))
            .and(header("Authorization", "Bearer test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::from_str::<serde_json::Value>(response_body).unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;

        let base_url = format!("{}/", mock_server.uri());

        let client = Client::new_with_base_url(&base_url).expect("Failed to create client");

        let messages = vec![Message {
            role: "user".to_string(),
            content: "Hi".to_string(),
        }];
        let result = client.chat("gpt-4", messages).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello world");
    }

    #[tokio::test]
    async fn test_chat_error_500() {
        let mock_server = MockServer::start().await;
        env::set_var("OPENAI_API_KEY", "test-key");

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
            .mount(&mock_server)
            .await;

        let base_url = format!("{}/", mock_server.uri());
        let client = Client::new_with_base_url(&base_url).expect("Failed to create client");

        let messages = vec![Message {
            role: "user".to_string(),
            content: "Hi".to_string(),
        }];
        let result = client.chat("gpt-4", messages).await;

        assert!(result.is_err());
        let err = result.err().unwrap().to_string();
        assert!(err.contains("500 Internal Server Error"));
    }
}
