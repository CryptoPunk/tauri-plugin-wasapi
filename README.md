# tauri-plugin-wasapi

Tauri 2 plugin for capturing audio from devices and applications using the
Windows Audio Session API (WASAPI). Audio data is streamed to the frontend
in real-time via Tauri's Channel IPC.

**Author:** Max Vohra <max-oss@seattlenetworks.com>

## Features

- **Device capture** — record from microphones and other capture devices
- **Loopback capture** — record system output audio (what you hear)
- **Application capture** — record audio from a specific process (Windows 10 20348+)
- **Device enumeration** — list all audio devices and their state
- **Process listing** — list running processes for application-specific capture
- **Real-time streaming** — PCM audio data streamed via Tauri Channel IPC

## Platform Support

| Platform | Supported |
|----------|-----------|
| Windows  | ✅        |
| macOS    | ❌        |
| Linux    | ❌        |
| Android  | ❌        |
| iOS      | ❌        |

## Installation

### Rust

Add the plugin to your Tauri app's `Cargo.toml`:

```toml
[dependencies]
tauri-plugin-wasapi = { path = "../tauri-plugin-wasapi" }
```

Register the plugin in your app:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_wasapi::init())
        .run(tauri::generate_context!())
        .expect("error while running application");
}
```

### Permissions

Add the plugin permissions to your `tauri.conf.json`:

```json
{
  "plugins": {
    "wasapi": {
      "permissions": ["wasapi:default"]
    }
  }
}
```

### Frontend (TypeScript/JavaScript)

```bash
npm install tauri-plugin-wasapi-api
# or link locally during development
```

## Usage

### List Audio Devices

```typescript
import { listDevices } from 'tauri-plugin-wasapi-api';

const devices = await listDevices();
for (const device of devices) {
  console.log(`${device.name} (${device.direction}) — ${device.state}`);
}
```

### List Processes

```typescript
import { listProcesses } from 'tauri-plugin-wasapi-api';

const processes = await listProcesses();
for (const proc of processes) {
  console.log(`PID ${proc.pid}: ${proc.name}`);
}
```

### Capture from Microphone

```typescript
import { startCapture, stopCapture } from 'tauri-plugin-wasapi-api';

await startCapture(
  { sessionId: 'mic-recording' },
  (event) => {
    switch (event.event) {
      case 'format':
        console.log('Format:', event.data);
        break;
      case 'data':
        // event.data.data is a raw PCM byte array (f32le interleaved)
        console.log(`Received ${event.data.frames} frames`);
        break;
      case 'error':
        console.error('Capture error:', event.data.message);
        break;
      case 'stopped':
        console.log('Capture stopped');
        break;
    }
  },
);

// Later, stop the capture:
await stopCapture('mic-recording');
```

### Loopback Capture (System Audio)

```typescript
await startCapture(
  { sessionId: 'loopback', loopback: true },
  (event) => { /* handle events */ },
);
```

### Application-Specific Capture

```typescript
import { listProcesses, startCapture } from 'tauri-plugin-wasapi-api';

const procs = await listProcesses();
const firefox = procs.find((p) => p.name.includes('firefox'));

if (firefox) {
  await startCapture(
    { sessionId: 'app-capture', processId: firefox.pid },
    (event) => { /* handle events */ },
  );
}
```

## Audio Format

All capture sessions produce **32-bit float, interleaved PCM** data.
The sample rate (default 48 kHz) and channel count (default 2) can be
configured per session. WASAPI's auto-convert feature handles resampling
from the hardware format transparently.

## Development

```bash
# Build the Rust plugin
cargo build

# Build the TypeScript bindings
npm install
npm run build
```

## License

MIT
