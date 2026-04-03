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
pub trait WasapiExt<R: Runtime> {
    fn wasapi(&self) -> &Wasapi<R>;
}

impl<R: Runtime, T: Manager<R>> crate::WasapiExt<R> for T {
    fn wasapi(&self) -> &Wasapi<R> {
        self.state::<Wasapi<R>>().inner()
    }
}

/// Initializes the plugin.
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
