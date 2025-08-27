NexusAI
An All-in-One AI Chat Hub
NexusAI is a real-time, WebSocket-powered chat application built with Rust that serves as a versatile interface for multiple large language models (LLMs). This project allows users to interact with and compare different AI models, including Gemini, OpenAI, and DeepSeek, all from a single, modern web interface.

The application is designed to be a robust and easy-to-use platform for experimenting with the capabilities of various AI services.

Key Features
Multi-Model Integration: Seamlessly connects to Gemini 1.5 Flash, OpenAI (GPT-3.5 Turbo), and DeepSeek Chat APIs.

Real-Time Communication: Utilizes WebSockets for instant, bi-directional communication between the client and server.

User-Driven Selection: A simple dropdown menu in the UI allows users to explicitly choose which AI model they want to chat with.

Enhanced User Experience: Includes a live loading indicator and disables inputs during API calls to provide immediate feedback and prevent duplicate requests.

Modular Architecture: The codebase is cleanly structured into separate modules (main.rs, ws_handler.rs, api/mod.rs) for improved readability and maintainability.

Getting Started
Prerequisites
Rust and Cargo: Ensure you have Rust and its package manager, Cargo, installed. You can install them from rustup.rs.

Installation
Clone the repository to your local machine:

Bash

git clone https://github.com/your-username/nexusai.git
cd nexusai
Create a .env file in the project's root directory and add your API keys.

Ini, TOML

GEMINI_API_KEY=your_gemini_api_key
OPENAI_API_KEY=your_openai_api_key
DEEPSEEK_API_KEY=your_deepseek_api_key
You can obtain these keys from their respective developer dashboards.

Running the Application
Once you have your API keys configured, you can run the application with a single command:

Bash

cargo run
The server will start on http://127.0.0.1:3000. Open this address in your web browser to start chatting.

Usage
Open the application in your browser.

Select your desired AI model from the dropdown menu.

Type a message in the input box and press Enter or click the "Send" button.

The application will display a "Loading..." message and then show the AI's response in the chat log.

Architecture
The project is structured into three main components:

main.rs: The entry point for the application. It handles server setup, environment variable loading, and request routing.

ws_handler.rs: Contains the core logic for the WebSocket connection. It manages incoming messages, parses them, and dispatches them to the appropriate API function.

api/mod.rs: A dedicated module for all API-related logic. It contains the functions (send_to_gemini, send_to_openai, send_to_deepseek) that handle HTTP requests to each AI provider.

This modular design makes it easy to add new features or integrate more AI models in the future.

License
This project is open-source and available under the MIT License.