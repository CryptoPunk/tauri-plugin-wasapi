# WASAPI Plugin Example

This directoy contains a Svelte-based Tauri application demonstrating the capabilities of the `tauri-plugin-wasapi`.

## Features Demonstrated

- **Audio Device Selection**: Enumerate and choose from available capture and render devices.
- **Process-Specific Capture**: List running OS processes and capture audio from a targeted application (e.g., Spotify, Chrome).
- **Real-time VU Meter**: Smooth, per-channel volume visualization.
- **Hierarchical Process Tree**: View processes organized by their parent-child relationships.
- **Custom UI Styling**: Premium dark-mode interface with sleek dropdowns and interactive elements.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Bun](https://bun.sh/)
- Windows 10/11 (WASAPI is a Windows-only API)

### Installation

1. Install dependencies:
   ```bash
   bun install
   ```

2. Run the application in development mode:
   ```bash
   bun run tauri dev
   ```

## Project Structure

- `src/`: Svelte frontend (UI, logic, VU meter, process tree).
- `src-tauri/`: Rust backend (tauri setup, wasapi plugin integration).
- `FILES.md`: Detailed project structure for this example.

## Technical Details

The frontend communicates with the Rust backend via Tauri's Channel IPC. Audio data is received as 32-bit float PCM chunks, which are then processed for visualization.

For more information on the core plugin, see the [root README](../../README.md).
