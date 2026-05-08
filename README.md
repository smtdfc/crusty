# Crusty Agent

Crusty Agent is an AI Agent built with Rust that integrates the [9router](https://github.com/decolua/9router) proxy. It automatically manages the configuration and connection through the 9router proxy.

## ⚙️ System Requirements
- **Rust & Cargo** (edition 2024) - [Rust Installation Guide](https://www.rust-lang.org/tools/install)
- **Node.js & npm** - Required for Crusty Agent to automatically install and run [`9router`](https://github.com/decolua/9router).

## 🛠️ Installation & Build

1. Clone the repository:
   ```bash
   git clone https://github.com/smtdfc/crusty
   cd crusty
   ```
2. Build the application:
   ```bash
   cargo build --release
   ```
   The executable file will be located at `target/release/crusty`.

## 📖 Usage Instructions

Crusty Agent comes with 2 main commands: `setup` and `chat`.

### 1. Initial Setup (`setup`)
Before chatting, you need to set up the basic configuration. Run the command:

```bash
cargo run -- setup
# Or: crusty setup
```

**This process will:**
- Check and automatically install `9router` via npm if it is not already installed.
- Ask you to enter a port to run `9router`.
- Create and save the configuration file in your operating system's default config directory (e.g., `%APPDATA%\crusty\config\config.json` on Windows, `~/.config/crusty/config.json` on Linux).

### 2. Start Chatting (`chat start`)
After the setup is complete, you can start the agent using the command:

```bash
cargo run -- chat start
# Or: crusty chat start
```

**This command will:**
- Automatically ensure that `9router` is running on your configured port.
- Display a welcome screen and open the chat interface.
- Allow you to type your message and press Enter to chat with the AI.

## 🤝 How to Contribute

Contributions are always welcome! Here are some ways you can contribute to the project:

1. **Report bugs:** If you find a bug, please create an issue on GitHub describing the problem.
2. **Suggest features:** If you have an idea for a new feature, feel free to open an issue or start a discussion.
3. **Submit Pull Requests:** 
   - Fork the repository.
   - Create a new branch for your feature or bug fix (`git checkout -b feature/your-feature-name`).
   - Commit your changes (`git commit -m 'Add some feature'`).
   - Push to the branch (`git push origin feature/your-feature-name`).
   - Open a Pull Request.

Please ensure your code passes all checks and follows the existing style before submitting a PR.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
