# FILES.md

Project structure for `tauri-plugin-wasapi`.

## Rust Source (`src/`)

| File          | Description                                                         |
|---------------|---------------------------------------------------------------------|
| `lib.rs`      | Plugin entrypoint — registers commands and manages plugin state     |
| `commands.rs` | Tauri command handlers (list_devices, list_processes, start/stop)   |
| `desktop.rs`  | Desktop implementation — WASAPI capture logic and session manager   |
| `mobile.rs`   | Mobile stub — all methods return `Error::Unsupported`               |
| `models.rs`   | Shared data models (AudioDevice, StreamEvent, etc.)                 |
| `error.rs`    | Error types                                                         |

## Frontend (`guest-js/`)

| File       | Description                                                    |
|------------|----------------------------------------------------------------|
| `index.ts` | TypeScript API — listDevices, listProcesses, startCapture, etc |

## Configuration

| File                       | Description                             |
|----------------------------|-----------------------------------------|
| `Cargo.toml`               | Rust crate configuration and deps       |
| `build.rs`                 | Tauri plugin build script               |
| `package.json`             | NPM package configuration               |
| `tsconfig.json`            | TypeScript compiler configuration        |
| `rollup.config.js`         | Rollup bundler config for guest-js      |
| `permissions/default.toml` | Default plugin permissions              |
