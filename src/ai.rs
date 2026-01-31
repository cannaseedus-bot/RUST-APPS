use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Instant;
use tokio::time::Duration;
use log::info;

#[derive(Debug, Clone)]
pub enum AIModelType {
    Phi3Mini,
    Phi3Small,
    Phi3Medium,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub content: String,
    pub tokens: usize,
    pub time_ms: u64,
    pub model: String,
}

#[derive(Debug)]
pub struct AIModel {
    model_type: AIModelType,
    pub model_path: Option<PathBuf>,
    pub context_size: usize,
    loaded: bool,
}

impl AIModel {
    pub async fn new(model_name: &str) -> Result<Self> {
        let model_type = match model_name.to_lowercase().as_str() {
            "phi-3-mini" => AIModelType::Phi3Mini,
            "phi-3-small" => AIModelType::Phi3Small,
            "phi-3-medium" => AIModelType::Phi3Medium,
            custom => AIModelType::Custom(custom.to_string()),
        };

        Ok(Self {
            model_type,
            model_path: None,
            context_size: 4096,
            loaded: false,
        })
    }

    pub async fn load(&mut self) -> Result<()> {
        if self.loaded {
            return Ok(());
        }
        info!("Loading AI model: {:?}", self.model_type);
        tokio::time::sleep(Duration::from_millis(200)).await;
        self.loaded = true;
        Ok(())
    }

    pub async fn generate(&mut self, prompt: &str, max_tokens: usize) -> Result<AIResponse> {
        self.load().await?;
        let start_time = Instant::now();
        let response = match self.model_type {
            AIModelType::Phi3Mini | AIModelType::Phi3Small | AIModelType::Phi3Medium => {
                self.generate_phi3_response(prompt)
            }
            AIModelType::Custom(_) => self.generate_generic_response(prompt),
        };
        let elapsed = start_time.elapsed();
        Ok(AIResponse {
            content: response,
            tokens: max_tokens.min(500),
            time_ms: elapsed.as_millis() as u64,
            model: self.model_type.to_string(),
        })
    }

    fn generate_phi3_response(&self, prompt: &str) -> String {
        if prompt.to_lowercase().contains("login") {
            "// Generated login component placeholder".to_string()
        } else if prompt.to_lowercase().contains("card") {
            "// Generated card component placeholder".to_string()
        } else {
            self.generate_generic_response(prompt)
        }
    }

    fn generate_generic_response(&self, prompt: &str) -> String {
        format!("// Generated code based on prompt: {}", prompt)
    }
}

impl ToString for AIModelType {
    fn to_string(&self) -> String {
        match self {
            AIModelType::Phi3Mini => "phi-3-mini".to_string(),
            AIModelType::Phi3Small => "phi-3-small".to_string(),
            AIModelType::Phi3Medium => "phi-3-medium".to_string(),
            AIModelType::Custom(name) => name.clone(),
        }
    }
}
