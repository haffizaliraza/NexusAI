use dotenv::dotenv;
use std::net::SocketAddr;
use std::env;
use axum::{
    extract::ws::WebSocketUpgrade,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

mod ws_handler;
mod api;

/// Shared application state
#[derive(Clone)]
struct AppState {
    tx: tokio::sync::broadcast::Sender<String>,
    gemini_api_key: String,
    openai_api_key: String,
    deepseek_api_key: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let gemini_api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let deepseek_api_key = env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY must be set");

    let (tx, _rx) = tokio::sync::broadcast::channel::<String>(256);
    let state = AppState { tx, gemini_api_key, openai_api_key, deepseek_api_key };

    // Build router
    let app = Router::new()
        .route("/", get(index))
        .route("/ws", get(upgrade_ws))
        .with_state(state);

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    println!("📡 WebSocket chat running on http://{addr}  (WS: ws://{addr}/ws)");

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

/// Simple HTML chat page
async fn index() -> impl IntoResponse {
    Html(
        r#"
<!doctype html>
<html>
  <head>
    <meta charset="utf-8"/>
    <title>NexusAI</title>
    <style>
      :root {
        --bg-color: #f0f2f5;
        --card-bg: #ffffff;
        --border-color: #e0e0e0;
        --text-color: #333;
        --primary-color: #007bff;
        --shadow-color: rgba(0, 0, 0, 0.1);
      }
      body {
        font: 16px/1.4 system-ui, sans-serif;
        margin: 0;
        padding: 2rem;
        display: flex;
        flex-direction: column;
        align-items: center;
        background-color: var(--bg-color);
        color: var(--text-color);
      }
      h1 {
        color: var(--primary-color);
        margin-bottom: 1.5rem;
      }
      #loading-indicator {
        display: none; /* Hidden by default */
        text-align: center;
        font-style: italic;
        color: #888;
        margin-top: 10px;
      }
      .chat-container {
        width: 100%;
        max-width: 600px;
        background-color: var(--card-bg);
        border: 1px solid var(--border-color);
        border-radius: 12px;
        box-shadow: 0 4px 12px var(--shadow-color);
        padding: 1.5rem;
      }
      #log {
        height: 400px;
        overflow-y: auto;
        padding: 1rem;
        border: 1px solid var(--border-color);
        border-radius: 8px;
        margin-bottom: 1rem;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
      }
      #log div {
        padding: 0.5rem 1rem;
        border-radius: 20px;
        max-width: 80%;
        word-wrap: break-word;
        box-shadow: 0 1px 3px rgba(0,0,0,0.08);
        transition: transform 0.2s ease-in-out, opacity 0.2s ease-in-out;
        opacity: 0.9;
      }
      #log div:hover {
        transform: translateY(-2px);
        opacity: 1;
      }
      #log div:first-child {
        background-color: #e9ecef;
        align-self: flex-start;
      }
      #log div:nth-child(2n) {
        background-color: #d1e7dd;
        align-self: flex-end;
      }
      #row {
        display:flex;
        gap: 0.75rem;
        align-items: center;
      }
      input, button, select {
        font: inherit;
        padding: 0.75rem 1rem;
        border: 1px solid var(--border-color);
        border-radius: 8px;
        transition: all 0.2s ease-in-out;
      }
      input {
        flex-grow: 1;
      }
      input:focus {
        border-color: var(--primary-color);
        outline: none;
        box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.25);
      }
      button {
        cursor: pointer;
        background-color: var(--primary-color);
        color: white;
        border: none;
      }
      button:hover {
        background-color: #0056b3;
      }
    </style>
  </head>
  <body>
    <div class="chat-container">
    <h1>NexusAI</h1>
    <h4>Rust LLM Switchboard</h4>
    <div id="log"></div>
    <div id="loading-indicator">Loading...</div>
    <div id="row">
      <select id="model-select">
        <option value="gemini">Gemini</option>
        <option value="openai">OpenAI</option>
        <option value="deepseek">DeepSeek</option>
      </select>
      <input id="msg" placeholder="Type a message and hit Enter…" autofocus />
      <button id="send">Send</button>
    </div>
  </div>
    <script>
      const log = document.getElementById('log');
      const modelSelect = document.getElementById('model-select');
      const msg = document.getElementById('msg');
      const btn = document.getElementById('send');
      const ws = new WebSocket(`ws://${location.host}/ws`);

      function append(t) {
        const div = document.createElement('div');
        div.textContent = t;
        log.appendChild(div);
        log.scrollTop = log.scrollHeight;
      }

      ws.onopen = () => append('✅ connected');
      ws.onclose = () => append('❌ disconnected');
      ws.onerror = () => append('⚠️ error');
      ws.onmessage = (ev) => append(ev.data);

      function send() {
        const text = msg.value.trim();
        const model = modelSelect.value;
        if (!text) return;
        
        const messageObject = {
            model: model,
            text: text,
        };
        ws.send(JSON.stringify(messageObject));
        msg.value = '';
      }

      btn.onclick = send;
      msg.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') send();
      });
    </script>
  </body>
</html>
"#,
    )
}

/// WebSocket upgrade handler
async fn upgrade_ws(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| ws_handler::handle_socket(socket, state))
}

// Struct for deserializing the incoming JSON message
#[derive(Deserialize)]
struct ChatMessage {
    model: String,
    text: String,
}