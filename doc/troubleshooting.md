# Troubleshooting

Common issues and solutions when working with the `tauri-plugin-wasapi`.

## Application Capture Fails

> [!IMPORTANT]
> Application-specific loopback capture (capturing audio from a single PID) requires **Windows 10 Build 20348 (SDK 10.0.20348.0)** or later.

- **Check Windows Version**: If you are on an older version of Windows 10, application capture will fail with an error. Use system-wide loopback instead.
- **Process Termination**: If the target process terminates, the capture session will end and send an `error` or `stopped` event.
- **Permissions**: Ensure your application has the necessary permissions to access information about other processes.

## "Access Denied" or "Device Not Found"

- **Privacy Settings**: On Windows 10/11, check **Settings > Privacy & security > Microphone** and ensure "Microphone access" and "Let desktop apps access your microphone" are turned **On**.
- **Exclusive Mode**: If another application has taken exclusive control of the audio device, WASAPI may return an "Access Denied" error. The plugin uses "Shared Mode" to avoid this, but some hardware drivers may still exhibit this behavior.
- **Disconnected Device**: Ensure the `deviceId` you are using is still valid and the device is plugged in.

## Audio Latency or Stuttering

- **CPU Pressure**: High CPU usage can cause the background capture thread to miss buffer events, leading to stuttering in the audio data.
- **IPC Overhead**: Sending raw PCM data over Tauri's JSON-based IPC `Channel` has some overhead. For very high-frequency visualizations, consider downsampling the audio data before sending it to the frontend, or increasing the chunk size.
- **Buffer Size**: The default chunk size is 4096 frames (~85ms). If you need lower latency, you may need to modify `chunk_frames` in `src/desktop.rs` (though this requires a Rust rebuild).

## "Capture Already Exists"

- **Session IDs**: Every `startCapture` call must have a unique `sessionId`. If you try to start a new session with an ID that is already active, it will fail.
- **Leaked Sessions**: If your frontend app reloads without calling `stopCapture`, the Rust-side session might still be running. The plugin state is managed by the Tauri `Manager`, so it persists across frontend reloads until the main window is closed or the app exits.

## COM Error (0x800401F0)

- **Initialization**: This error usually means COM has not been initialized on the current thread. The plugin handles this automatically inside the `capture_thread` by calling `initialize_mta()`. If you encounter this, ensure that you are not calling WASAPI methods from a custom thread without first initializing COM.
