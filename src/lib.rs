use std::fmt;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://integrate.api.nvidia.com/v1/chat/completions";

#[derive(Debug, Serialize, Deserialize)]
pub enum LLModel {
    #[serde(rename = "aisingapore/sea-lion-7b-instruct")]
    AiSingaporeSeaLion7BInstruct,
    #[serde(rename = "databricks/dbrx-instruct")]
    DatabricksDbrxInstruct,
    #[serde(rename = "google/gemma-7b")]
    GoogleGemma7B,
    #[serde(rename = "google/gemma-2b")]
    GoogleGemma2B,
    #[serde(rename = "google/codegemma-1.1-7b")]
    GoogleCodeGemma1_17B,
    #[serde(rename = "google/codegemma-7b")]
    GoogleCodeGemma7B,
    #[serde(rename = "google/recurrentgemma-2b")]
    GoogleRecurrentGemma2B,
    #[serde(rename = "ibm/granite-34b-code-instruct")]
    IbmGranite34BCodeInstruct,
    #[serde(rename = "ibm/granite-8b-code-instruct")]
    IbmGranite8BCodeInstruct,
    #[serde(rename = "mediatek/breeze-7b-instruct")]
    MediatekBreeze7BInstruct,
    #[serde(rename = "meta/codellama-70b")]
    MetaCodellama70B,
    #[serde(rename = "meta/llama2-70b")]
    MetaLlama2_70B,
    #[serde(rename = "meta/llama3-8b-instruct")]
    MetaLlama3_8B,
    #[serde(rename = "meta/llama3-70b-instruct")]
    MetaLlama3_70B,
    #[serde(rename = "microsoft/phi-3-medium-4k-instruct")]
    MicrosoftPhi3Medium4KInstruct,
    #[serde(rename = "microsoft/phi-3-mini")]
    MicrosoftPhi3Mini128KInstruct,
    #[serde(rename = "microsoft/phi-3-mini-4k-instruct")]
    MicrosoftPhi3Mini4KInstruct,
    #[serde(rename = "microsoft/phi-3-small-128k-instruct")]
    MicrosoftPhi3Small128KInstruct,
    #[serde(rename = "microsoft/phi-3-small-8k-instruct")]
    MicrosoftPhi3Small8KInstruct,
    #[serde(rename = "mistralai/mistral-7b-instruct-v0.2")]
    MistralAiMixtral7BInstruct,
    #[serde(rename = "mistralai/mixtral-8x7b-instruct-v0.1")]
    MistralAiMixtral8x7BInstruct,
    #[serde(rename = "mistralai/mixtral-8x22b-instruct-v0.1")]
    MistralAiMixtral8x22BInstruct,
    #[serde(rename = "mistralai/mistral-large")]
    MistralAiMistralLarge,
    #[serde(rename = "seallms/seallm-7b-v2.5")]
    SealLlmsSealllm7BV2_5,
    #[serde(rename = "snowflake/arctic")]
    SnowFlakeArctic,
}

impl fmt::Display for LLModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LLModel::AiSingaporeSeaLion7BInstruct => write!(f, "aisingapore/sea-lion-7b-instruct"),
            LLModel::DatabricksDbrxInstruct => write!(f, "databricks/dbrx-instruct"),
            LLModel::GoogleGemma7B => write!(f, "google/gemma-7b"),
            LLModel::GoogleGemma2B => write!(f, "google/gemma-2b"),
            LLModel::GoogleCodeGemma1_17B => write!(f, "google/codegemma-1.1-7b"),
            LLModel::GoogleCodeGemma7B => write!(f, "google/codegemma-7b"),
            LLModel::GoogleRecurrentGemma2B => write!(f, "google/recurrentgemma-2b"),
            LLModel::IbmGranite34BCodeInstruct => write!(f, "ibm/granite-34b-code-instruct"),
            LLModel::IbmGranite8BCodeInstruct => write!(f, "ibm/granite-8b-code-instruct"),
            LLModel::MediatekBreeze7BInstruct => write!(f, "mediatek/breeze-7b-instruct"),
            LLModel::MetaCodellama70B => write!(f, "meta/codellama-70b"),
            LLModel::MetaLlama2_70B => write!(f, "meta/llama2-70b"),
            LLModel::MetaLlama3_8B => write!(f, "meta/llama3-8b-instruct"),
            LLModel::MetaLlama3_70B => write!(f, "meta/llama3-70b-instruct"),
            LLModel::MicrosoftPhi3Medium4KInstruct => write!(f, "microsoft/phi-3-medium-4k-instruct"),
            LLModel::MicrosoftPhi3Mini128KInstruct => write!(f, "microsoft/phi-3-mini"),
            LLModel::MicrosoftPhi3Mini4KInstruct => write!(f, "microsoft/phi-3-mini-4k-instruct"),
            LLModel::MicrosoftPhi3Small8KInstruct => write!(f, "microsoft/phi-3-small-8k-instruct"),
            LLModel::MistralAiMixtral7BInstruct => write!(f, "mistralai/mistral-7b-instruct-v0.2"),
            LLModel::MistralAiMixtral8x7BInstruct => write!(f, "mistralai/mixtral-8x7b-instruct-v0.1"),
            LLModel::MistralAiMixtral8x22BInstruct => write!(f, "mistralai/mixtral-8x22b-instruct-v0.1"),
            LLModel::MistralAiMistralLarge => write!(f, "mistralai/mistral-large"),
            LLModel::SealLlmsSealllm7BV2_5 => write!(f, "seallms/seallm-7b-v2.5"),
            LLModel::SnowFlakeArctic => write!(f, "snowflake/arctic"),
            _ => write!(f, "Unknown"),
        }
    }
}

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

#[derive(Serialize, Deserialize, Debug)]
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
    pub messages: Vec<Message>,
}

impl Payload {
    pub fn model(&mut self, model: String) -> &mut Self {
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

    pub fn messages(&mut self, messages: Message) -> &mut Self {
        self.messages = vec![messages];
        self
    }

    pub fn add_message(&mut self, role: &str, content: &str) -> &mut Self {
        self.messages.push(Message {
            role: role.to_string(),
            content: content.to_string(),
        });
        self
    }
}

impl Default for Payload {
    fn default() -> Self {
        Self {
            model: "mistralai/mistral-large".to_string(),
            max_tokens: 1024,
            stream: false,
            temperature: 0.3,
            top_p: 1.0,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
            seed: None,
            stop: None,
            messages: vec![Message {
                role: "system".to_string(),
                content: "Hello!".to_string(),
            }],
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
