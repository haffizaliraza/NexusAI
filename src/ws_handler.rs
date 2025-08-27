use axum::{
    extract::ws::{Message, WebSocket},
};
use futures_util::{SinkExt, StreamExt};
use serde_json::from_str;
use crate::{AppState, ChatMessage, api}; // Import necessary items

/// Handles each WebSocket connection
pub async fn handle_socket(socket: WebSocket, state: AppState) {
    let mut rx = state.tx.subscribe();
    let (mut sender, mut receiver) = socket.split();

    let mut write_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let read_state = state.clone();
    let mut read_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                let chat_message: ChatMessage = match from_str(&text) {
                    Ok(msg) => msg,
                    Err(e) => {
                        eprintln!("Failed to parse JSON message: {}", e);
                        continue;
                    }
                };
                
                let user_text = chat_message.text;
                let model_choice = chat_message.model;

                let _ = read_state.tx.send(format!("You: {user_text}"));
                
                let response = match model_choice.as_str() {
                    "gemini" => api::send_to_gemini(&user_text, &read_state.gemini_api_key).await,
                    "openai" => api::send_to_openai(&user_text, &read_state.openai_api_key).await,
                    "deepseek" => api::send_to_deepseek(&user_text, &read_state.deepseek_api_key).await,
                    _ => Ok(String::from("Please select a valid AI model.")),
                };

                match response {
                    Ok(ai_response) => {
                        let _ = read_state.tx.send(format!("🤖 {}: {}", model_choice, ai_response));
                    }
                    Err(e) => {
                        eprintln!("API error: {}", e);
                        let _ = read_state.tx.send(format!("🤖 Error with {}: {}", model_choice, e));
                    }
                }
            }
        }
    });

    tokio::select! {
        _ = &mut write_task => { read_task.abort(); }
        _ = &mut read_task => { write_task.abort(); }
    }
}