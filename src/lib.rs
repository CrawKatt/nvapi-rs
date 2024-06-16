use reqwest::Client;
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://integrate.api.nvidia.com/v1/chat/completions";

#[derive(Deserialize, Debug)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
    pub completion_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub model: String,
    pub max_tokens: u32,
    pub stream: bool,
    pub temperature: f64,
    pub top_p: f64,
    pub frequency_penalty: f64,
    pub presence_penalty: f64,
    pub seed: Option<u64>,
    pub stop: Option<String>,
    pub messages: String
}

impl Payload {
    pub fn model(&mut self, model: &str) -> &mut Self {
        self.model = model.to_string();
        self
    }

    pub fn max_tokens(&mut self, max_tokens: u32) -> &mut Self {
        self.max_tokens = max_tokens;
        self
    }

    pub fn stream(&mut self, stream: bool) -> &mut Self {
        self.stream = stream;
        self
    }

    pub fn temperature(&mut self, temperature: f64) -> &mut Self {
        self.temperature = temperature;
        self
    }

    pub fn top_p(&mut self, top_p: f64) -> &mut Self {
        self.top_p = top_p;
        self
    }

    pub fn frequency_penalty(&mut self, frequency_penalty: f64) -> &mut Self {
        self.frequency_penalty = frequency_penalty;
        self
    }

    pub fn presence_penalty(&mut self, presence_penalty: f64) -> &mut Self {
        self.presence_penalty = presence_penalty;
        self
    }

    pub fn seed(&mut self, seed: u64) -> &mut Self {
        self.seed = Some(seed);
        self
    }

    pub fn stop(&mut self, stop: &str) -> &mut Self {
        self.stop = Some(stop.to_string());
        self
    }

    pub fn messages(&mut self, messages: &str) -> &mut Self {
        self.messages = messages.to_string();
        self
    }
}

impl Default for Payload {
    fn default() -> Self {
        Self {
            model: "mistralai/mixtral-8x7b-instruct-v0.1".to_string(),
            max_tokens: 1024,
            stream: false,
            temperature: 0.3,
            top_p: 1.0,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
            seed: None,
            stop: None,
            messages: "hola".to_string(),
        }
    }
}

pub async fn send_request(api_key: &str, payload: &Payload) -> Result<Response, reqwest::Error> {
    let client = Client::new();
    let response = client.post(API_URL)
        .json(payload)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {api_key}"))
        .send()
        .await?;

    let response_text = response.text().await?;
    let response_object = serde_json::from_str(&response_text).unwrap();

    Ok(response_object)
}
