use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};
use std::error::Error;

/// Sends a message to the Gemini API and returns the response text.
pub async fn send_to_gemini(
    prompt: &str,
    api_key: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", api_key);
    
    let body = json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }]
    });

    let res = client.post(&url).json(&body).send().await?;
    let status = res.status();
    if !status.is_success() {
        let error_body = res.text().await?;
        return Err(format!("Gemini API returned an error. Status: {}. Body: {}", status, error_body).into());
    }

    let parsed_json: Value = res.json().await?;
    let text = parsed_json["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or("Could not parse text from Gemini response")?
        .to_string();

    Ok(text)
}

/// Sends a message to the OpenAI API and returns the response text.
pub async fn send_to_openai(
    prompt: &str,
    api_key: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let url = "https://api.openai.com/v1/chat/completions";

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse()?);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);

    let body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {
                "role": "system",
                "content": "You are a helpful assistant."
            },
            {
                "role": "user",
                "content": prompt
            }
        ]
    });

    let res = client.post(url).headers(headers).json(&body).send().await?;
    let status = res.status();
    if !status.is_success() {
        let error_body = res.text().await?;
        return Err(format!("OpenAI API returned an error. Status: {}. Body: {}", status, error_body).into());
    }

    let parsed_json: Value = res.json().await?;
    let text = parsed_json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("Could not parse text from OpenAI response")?
        .to_string();

    Ok(text)
}

/// Sends a message to the DeepSeek API and returns the response text.
pub async fn send_to_deepseek(
    prompt: &str,
    api_key: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let url = "https://api.deepseek.com/v1/chat/completions";

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse()?);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);

    let body = json!({
        "model": "deepseek-chat",
        "messages": [
            {
                "role": "system",
                "content": "You are a helpful assistant."
            },
            {
                "role": "user",
                "content": prompt
            }
        ]
    });

    let res = client.post(url).headers(headers).json(&body).send().await?;
    let status = res.status();
    if !status.is_success() {
        let error_body = res.text().await?;
        return Err(format!("DeepSeek API returned an error. Status: {}. Body: {}", status, error_body).into());
    }

    let parsed_json: Value = res.json().await?;
    let text = parsed_json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("Could not parse text from DeepSeek response")?
        .to_string();

    Ok(text)
}