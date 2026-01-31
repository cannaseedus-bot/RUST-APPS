# Nexus Studio AI (Rust CLI)

High-performance Rust CLI scaffolding for Nexus Studio AI, including optional web UI and AI generation stubs.

## Features

- 🧰 CLI commands for project scaffolding, component generation, builds, and deployment
- 🤖 AI model stub for code generation flows
- 🌐 Optional Warp-based web interface with REST and WebSocket endpoints
- 📦 Config and template helpers for local development
- 🖥️ PowerShell TUI and installer scripts for Windows users

## Getting Started

### Prerequisites

- Rust 1.70+ (recommended)

### Build

```bash
cargo build
```

### Run

```bash
# CLI
cargo run -- nexus --help

# Web UI (requires web feature)
cargo run --features web -- nexus web --port 8080
```

## PowerShell TUI (Windows)

The repository includes two PowerShell scripts for an interactive TUI and an installer.

```powershell
# Launch the TUI directly
./nexus-tui.ps1

# Run the installer
./nexus-install.ps1
```

## Project Structure

```text
src/
  ai.rs         AI model stub
  builder.rs    Build pipeline stub
  commands.rs   CLI command handlers
  config.rs     Config loader/saver
  main.rs       CLI entrypoint
  project.rs    Project scaffolding helpers
  types.rs      CLI types and subcommands
  web.rs        Warp web server and websocket handlers
templates/
  index.html    Web landing page
nexus-tui.ps1   PowerShell TUI
nexus-install.ps1 PowerShell installer
```

## Notes

- The AI layer is a placeholder to be wired into a real model provider.
- Use the `web` feature flag to enable the Warp server integration.

## License

MIT
