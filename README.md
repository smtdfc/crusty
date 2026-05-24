# Crusty Agent

Crusty is a lightweight AI agent written in Rust that can run through an AI proxy/router or connect directly to an OpenAI‑compatible provider. It provides a simple CLI to configure, start/stop proxies, manage providers, and run chat sessions.

### Key features

- Two run modes: **Proxy mode** (use a local/remote proxy/router) and **Provider mode** (use an OpenAI‑compatible provider).
- Manage multiple proxies and providers, start/stop proxy processes, and open proxy dashboards from the CLI.
- Automatic installation of proxy packages (via npm) when required.

### Supported proxies/routers

- 9router — https://github.com/decolua/9router
- OmniRoute — https://github.com/diegosouzapw/OmniRoute

### System requirements

- Rust & Cargo (edition 2024) — https://www.rust-lang.org/tools/install
- Node.js & npm — required for installing and running proxy packages

### Installation & build

1. Clone the repository:

```bash
git clone https://github.com/smtdfc/crusty
cd crusty
```

2. Build the application:

```bash
cargo build
cargo build --release
```

The release executable will be available at `target/release/crusty`.

### Quick start

1. Run the setup wizard to configure a provider or proxy:

```bash
crusty setup
```

2. If you chose Proxy mode, configure and optionally start the proxy (local):

```bash
crusty proxy start
```

3. Start the chat session:

```bash
crusty start
# or: crusty start --chat   # jump directly into chat mode
```

### Modes

#### Provider mode

Provider mode allows Crusty to use a direct OpenAI‑compatible provider (OpenAI, Anthropic, or a custom provider). Add providers via the setup wizard or using the provider CLI commands. When Provider mode is active, `crusty start` will use the configured provider and model.

**Provider commands:**

```bash
crusty provider add
crusty provider list
crusty provider remove <name>
crusty provider switch <name>
```

#### Proxy mode

Proxy mode uses a proxy/router (local or remote) as the API endpoint for the agent. Proxy setup may ask for platform‑specific details (host, port, API key, model name).

**Proxy commands:**

```bash
crusty setup             # run setup and add a proxy
crusty proxy start       # start the active local proxy
crusty proxy stop        # stop the active local proxy
crusty proxy dashboard   # open the proxy dashboard in the browser
crusty proxy list        # list configured proxies
```

### CLI reference (common commands)

- `crusty setup` — run the interactive setup wizard
- `crusty start [--chat]` — start the agent; `--chat` jumps directly into the chat UI
- `crusty proxy start|stop|dashboard` — control the active proxy
- `crusty provider add|list|remove|switch` — manage providers

### Troubleshooting

- If the proxy is reported offline when starting the agent, ensure the proxy process is running and the host/port are correct.
- If npm package installation fails, verify `node` and `npm` are installed and have network access.

### Contributing

Contributions are welcome:

1. Fork the repository
2. Create a branch for your change: `git checkout -b feature/your-feature`
3. Commit and push
4. Open a Pull Request

Please ensure tests/build succeed before opening a PR.

### License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
