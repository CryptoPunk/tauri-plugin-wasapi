# FILES.md (Example App)

Project structure for the `tauri-app` example, demonstrating the use of `tauri-plugin-wasapi`.

## Frontend (`src/`)

This directory contains the Svelte-based frontend implementation.

| File          | Description                                                         |
|---------------|---------------------------------------------------------------------|
| `App.svelte`  | Main application component — contains UI for device selection and VU meter |
| `main.js`     | Entrypoint for the Svelte application                               |
| `style.css`   | Global styles (including custom styling for selectors)              |

## Tauri Backend (`src-tauri/`)

This directory contains the Rust backend for the Tauri application.

| File               | Description                                                     |
|--------------------|-----------------------------------------------------------------|
| `src/main.rs`      | Application entrypoint for the Rust backend                     |
| `src/lib.rs`       | Backend logic — initializes the WASAPI plugin                   |
| `tauri.conf.json`  | Main Tauri configuration (window settings, security, plugins)   |
| `Cargo.toml`       | Rust dependencies and configuration                             |
| `capabilities/`    | Directory for application permission sets                       |

## Root Configuration

| File             | Description                                     |
|------------------|-------------------------------------------------|
| `package.json`   | NPM/Bun package configuration and dependencies  |
| `vite.config.js` | Vite configuration for the Svelte frontend      |
| `index.html`     | Main HTML entry point for the browser           |
| `README.md`      | Overview of the example application             |
| `FILES.md`       | This file — project structure overview          |
