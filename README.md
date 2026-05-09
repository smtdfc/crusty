# Crusty Agent

Crusty is an AI Agent built with Rust that integrates the AI proxy.

## Support AI Proxy

- [9router](https://github.com/decolua/9router).

## System Requirements

- **Rust & Cargo** (edition 2024) - [Rust Installation Guide](https://www.rust-lang.org/tools/install)
- **Node.js & npm** - Required for Crusty Agent to automatically install and run some tool

## Installation & Build

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

## Usage Instructions

### 1. Initial Setup (`setup`)

Before chatting, you need to set up the basic configuration. Run the command:

```bash
cargo run -- setup
# Or: crusty setup
```

### 2. Start proxy

```bash
cargo run -- proxy start
# Or: crusty proxy start
```

### 3. Start Chatting

After the setup is complete, you can start the agent using the command:

```bash
cargo run -- chat
# Or: crusty chat
```

**This command will:**

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

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
