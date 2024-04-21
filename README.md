# yt-observer

This template should help get you started developing with Tauri and Yew.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Setup development
Your system is missing dependencies (or they do not exist in $PATH):
╭───────────────┬────────────────────────────────────────────────╮
│ Tauri CLI     │ Run `cargo install tauri-cli`                  │
├───────────────┼────────────────────────────────────────────────┤
│ Trunk         │ Run `cargo install trunk`                      │
├───────────────┼────────────────────────────────────────────────┤
│ wasm32 target │ Run `rustup target add wasm32-unknown-unknown` │
╰───────────────┴────────────────────────────────────────────────╯

Make sure you have installed the prerequisites for your OS: https://tauri.app/v1/guides/getting-started/prerequisites, then run:
  cargo tauri dev
