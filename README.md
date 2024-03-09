# Tauri + Leptos

This template should help get you started developing with Tauri and Leptos.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Development

Make sure to install the tauri-cli in version 2!

    cargo install tauri-cli@2.0.0-beta.8

Make sure that `trunk` is installed:

    cargo install trunk

Make sure you have installed the prerequisites for your OS: <https://tauri.app/v1/guides/getting-started/prerequisites>, then run:

    cargo tauri android init
    cargo tauri ios init

For Desktop development, run:

    cargo tauri dev

For Android development, run:

    cargo tauri android dev

For iOS development, run:

    cargo tauri ios dev
