use tauri::{command, ipc::Channel, AppHandle, Runtime};

use crate::models::*;
use crate::Result;
use crate::WasapiExt;

/// List all available audio capture and render devices.
///
/// This command enumerates physical and virtual audio endpoints on the system,
/// providing their ID, name, direction, and current state.
#[command]
pub(crate) async fn list_devices<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<AudioDevice>> {
    app.wasapi().list_devices()
}

/// List running OS processes (for application-specific capture).
///
/// This command scans all running processes to provide PID, name, and parent PID,
/// which can be used to target a specific application for audio loopback capture.
#[command]
pub(crate) async fn list_processes<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<ProcessInfo>> {
    app.wasapi().list_processes()
}

/// Start an audio capture session.
///
/// This command spawns a dedicated background thread for WASAPI capture.
/// Audio data and lifecycle events are streamed to the frontend via the
/// provided `Channel`.
///
/// # Arguments
///
/// * `request` - Configuration for the capture session (device, loopback, process, etc.).
/// * `on_event` - A Tauri IPC channel for streaming `StreamEvent` objects.
#[command]
pub(crate) async fn start_capture<R: Runtime>(
    app: AppHandle<R>,
    request: StartCaptureRequest,
    on_event: Channel<StreamEvent>,
) -> Result<()> {
    app.wasapi().start_capture(request, on_event)
}

/// Stop a running capture session.
///
/// This command signals the background capture thread to stop and cleans up
/// all associated WASAPI resources.
#[command]
pub(crate) async fn stop_capture<R: Runtime>(
    app: AppHandle<R>,
    request: StopCaptureRequest,
) -> Result<()> {
    app.wasapi().stop_capture(&request.session_id)
}
