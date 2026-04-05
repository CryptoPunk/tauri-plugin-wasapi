//! WASAPI audio capture plugin for Tauri 2.0.
//!
//! This plugin provides low-level access to the Windows Audio Session API (WASAPI),
//! allowing for real-time capture of audio from input devices (microphones),
//! output devices (loopback), and specific OS processes.

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Wasapi;
#[cfg(mobile)]
use mobile::Wasapi;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`]
/// to access the WASAPI audio capture APIs.
///
/// This trait is automatically implemented for all types that implement [`tauri::Manager`].
///
/// # Example
///
/// ```rust
/// use tauri_plugin_wasapi::WasapiExt;
///
/// fn my_command<R: tauri::Runtime>(app: tauri::AppHandle<R>) {
///     let devices = app.wasapi().list_devices().unwrap();
/// }
/// ```
pub trait WasapiExt<R: Runtime> {
    /// Returns the managed instance of the WASAPI plugin state.
    fn wasapi(&self) -> &Wasapi<R>;
}

impl<R: Runtime, T: Manager<R>> crate::WasapiExt<R> for T {
    fn wasapi(&self) -> &Wasapi<R> {
        self.state::<Wasapi<R>>().inner()
    }
}

/// Initializes the WASAPI plugin.
///
/// This function registers the necessary command handlers and sets up the
/// platform-specific backend (desktop or mobile stub).
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("wasapi")
        .invoke_handler(tauri::generate_handler![
            commands::list_devices,
            commands::list_processes,
            commands::start_capture,
            commands::stop_capture,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let wasapi = mobile::init(app, api)?;
            #[cfg(desktop)]
            let wasapi = desktop::init(app, api)?;
            app.manage(wasapi);
            Ok(())
        })
        .build()
}
