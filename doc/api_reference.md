# API Reference

This document provides a detailed reference for all functions and interfaces exported by the `tauri-plugin-wasapi-api` frontend package.

## Functions

### `listDevices()`
Lists all available audio capture and render endpoints on the system.

- **Returns**: `Promise<AudioDevice[]>`
- **Example**:
  ```typescript
  import { listDevices } from 'tauri-plugin-wasapi-api';
  const devices = await listDevices();
  ```

---

### `listProcesses()`
Lists all currently running OS processes. This is primarily used to discover a PID for application-specific audio capture.

- **Returns**: `Promise<ProcessInfo[]>`
- **Example**:
  ```typescript
  import { listProcesses } from 'tauri-plugin-wasapi-api';
  const procs = await listProcesses();
  ```

---

### `startCapture(options, onEvent)`
Starts a new audio capture session. This starts a background thread in the Rust side that streams audio data to the provided callback.

- **Parameters**:
  - `options`: [`StartCaptureOptions`](#startcaptureoptions)
  - `onEvent`: `(event: StreamEvent) => void`
- **Returns**: `Promise<void>`
- **Example**:
  ```typescript
  import { startCapture } from 'tauri-plugin-wasapi-api';

  await startCapture(
    { sessionId: 'main-capture', loopback: true },
    (event) => {
      if (event.event === 'data') {
        // Handle audio data
      }
    }
  );
  ```

---

### `stopCapture(sessionId)`
Signals a running capture session to stop and releases all associated resources.

- **Parameters**:
  - `sessionId`: `string` (must match a previously started session)
- **Returns**: `Promise<void>`

---

## Interfaces

### `AudioDevice`
| Property | Type | Description |
|---|---|---|
| `id` | `string` | The unique WASAPI device identifier. |
| `name` | `string` | The human-readable name of the device. |
| `direction` | `'capture' \| 'render'` | The device direction. |
| `state` | `string` | The current operational state (e.g., 'active', 'disabled'). |

### `StartCaptureOptions`
| Property | Type | Default | Description |
|---|---|---|---|
| `sessionId` | `string` | (Required) | A unique session identifier. |
| `deviceId` | `string` | `null` | The WASAPI device ID. Omit for system default. |
| `loopback` | `boolean` | `false` | When true, captures system playback. |
| `processId` | `number` | `null` | The Process ID for application-specific capture. |
| `sampleRate` | `number` | `48000` | The desired sample rate in Hz. |
| `channels` | `number` | `2` | The desired channel count. |

### `StreamEvent`
A tagged union representing events sent from the capture stream.

- **`event: 'format'`**: Sent first. Contains [`AudioFormatInfo`](#audioformatinfo).
- **`event: 'data'`**: Sent periodically. Contains [`AudioChunk`](#audiochunk).
- **`event: 'error'`**: Sent if an error occurs. Contains `message`.
- **`event: 'stopped'`**: Sent when the session has stopped.

### `AudioChunk`
| Property | Type | Description |
|---|---|---|
| `sessionId` | `string` | The session identifier. |
| `data` | `number[]` | Raw PCM bytes (f32le, interleaved). |
| `sampleRate` | `number` | The sample rate of this data. |
| `channels` | `number` | The number of audio channels. |
| `frames` | `number` | The number of audio frames in this chunk. |
