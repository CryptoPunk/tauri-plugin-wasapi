use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("WASAPI error: {0}")]
    Wasapi(String),

    #[error("Capture session not found: {0}")]
    SessionNotFound(String),

    #[error("Capture session already exists: {0}")]
    SessionAlreadyExists(String),

    #[error("Capture error: {0}")]
    CaptureError(String),

    #[error("Platform not supported: WASAPI is Windows-only")]
    Unsupported,

    #[cfg(mobile)]
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
