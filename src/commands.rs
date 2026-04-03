use tauri::{command, ipc::Channel, AppHandle, Runtime};

use crate::models::*;
use crate::Result;
use crate::WasapiExt;

/// List all available audio capture and render devices.
#[command]
pub(crate) async fn list_devices<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<AudioDevice>> {
    app.wasapi().list_devices()
}

/// List running OS processes (for application-specific capture).
#[command]
pub(crate) async fn list_processes<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<ProcessInfo>> {
    app.wasapi().list_processes()
}

/// Start an audio capture session. Audio data is streamed to the
/// frontend via the `on_event` channel.
#[command]
pub(crate) async fn start_capture<R: Runtime>(
    app: AppHandle<R>,
    request: StartCaptureRequest,
    on_event: Channel<StreamEvent>,
) -> Result<()> {
    app.wasapi().start_capture(request, on_event)
}

/// Stop a running capture session.
#[command]
pub(crate) async fn stop_capture<R: Runtime>(
    app: AppHandle<R>,
    request: StopCaptureRequest,
) -> Result<()> {
    app.wasapi().stop_capture(&request.session_id)
}
