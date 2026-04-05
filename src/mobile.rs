use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_wasapi);

/// Initializes the mobile context (no-op stub).
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Wasapi<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("", "WasapiPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_wasapi)?;
    Ok(Wasapi(handle))
}

/// Access to the WASAPI APIs.
///
/// This is a stub implementation for mobile platforms (iOS and Android).
/// All methods return `Error::Unsupported` since WASAPI is a Windows-only technology.
pub struct Wasapi<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Wasapi<R> {
    /// Stub: returns `Error::Unsupported`.
    pub fn list_devices(&self) -> crate::Result<Vec<AudioDevice>> {
        Err(crate::Error::Unsupported)
    }

    /// Stub: returns `Error::Unsupported`.
    pub fn list_processes(&self) -> crate::Result<Vec<ProcessInfo>> {
        Err(crate::Error::Unsupported)
    }

    /// Stub: returns `Error::Unsupported`.
    pub fn start_capture(
        &self,
        _request: StartCaptureRequest,
        _on_event: tauri::ipc::Channel<StreamEvent>,
    ) -> crate::Result<()> {
        Err(crate::Error::Unsupported)
    }

    /// Stub: returns `Error::Unsupported`.
    pub fn stop_capture(&self, _session_id: &str) -> crate::Result<()> {
        Err(crate::Error::Unsupported)
    }
}
