NexusAI
An All-in-One AI Chat Hub

NexusAI is a real-time, WebSocket-powered chat application built in Rust, designed as a versatile interface for multiple Large Language Models (LLMs). It allows users to interact with and compare different AI models—such as Gemini, OpenAI, and DeepSeek—all from a single, modern web interface.

This platform is ideal for developers, researchers, and enthusiasts who want to experiment with and explore the capabilities of various AI services in one place.

<img width="1220" height="872" alt="image" src="https://github.com/user-attachments/assets/9a26697e-371d-4757-b2ea-745dc00f5f2f" />


✨ Key Features

Multi-Model Integration
Seamlessly connect to:

Gemini 1.5 Flash

OpenAI (GPT-3.5 Turbo)

DeepSeek Chat APIs

Real-Time Communication
Built with WebSockets for instant, bi-directional communication between the client and server.

User-Driven Selection
A simple dropdown menu in the UI allows users to select the AI model they want to chat with.

Enhanced User Experience

Live loading indicators

Disabled inputs during API calls (to prevent duplicate requests)

Modular Architecture
Code is organized into separate modules for clarity and maintainability:

main.rs

ws_handler.rs

api/mod.rs

🚀 Getting Started
Prerequisites

Rust
 and Cargo installed (via rustup.rs
)

🔧 Installation

Clone the repository:

git clone https://github.com/haffizaliraza/nexusai.git
cd nexusai


Create a .env file in the project root and add your API keys:

GEMINI_API_KEY=your_gemini_api_key
OPENAI_API_KEY=your_openai_api_key
DEEPSEEK_API_KEY=your_deepseek_api_key


You can obtain these keys from the respective developer dashboards.

▶ Running the Application

Start the server:

cargo run


The server will start at:
http://127.0.0.1:3000

Open this address in your browser to start chatting.

💬 Usage

Open the application in your browser.

Select your desired AI model from the dropdown menu.

Type your message and hit Enter or click Send.

A “Loading...” indicator will appear until the AI responds.

📂 Project Architecture

The codebase follows a clean modular design:

main.rs – Application entry point, server setup, routing, and environment configuration.

ws_handler.rs – WebSocket logic (manages connections, message handling, and dispatching to APIs).

api/mod.rs – API integrations for:

send_to_gemini()

send_to_openai()

send_to_deepseek()

This design makes it easy to add new features or integrate additional AI models.

📜 License

This project is licensed under the MIT License.
