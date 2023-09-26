use serde::{Serialize, Deserialize};
use reqwest::blocking::Client;

const MARKDOWN_API_ENDPOINT: &str = "https://api.github.com/markdown";

#[derive(Debug, Serialize, Deserialize)]
struct MarkdownRequest {
    text: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mode: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    context: String,
}

pub fn get_client() -> Client {
    let client = Client::builder()
        .user_agent("xevion-find-syntax")
        .build()
        .unwrap();

    client
}

pub fn get_markdown(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = get_client();
    
    let request = MarkdownRequest {
        text: text.to_string(),
        mode: "".to_string(),
        context: "".to_string()
    };

    let response = client.post(MARKDOWN_API_ENDPOINT).json(&request).send()?;
    let markdown = response.text()?;

    Ok(markdown)
}