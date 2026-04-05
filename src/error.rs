use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

/// The error type for the WASAPI plugin.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An unexpected I/O error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// A low-level WASAPI error returned by the Windows API.
    #[error("WASAPI error: {0}")]
    Wasapi(String),

    /// The requested capture session was not found.
    #[error("Capture session not found: {0}")]
    SessionNotFound(String),

    /// A capture session with the same ID already exists.
    #[error("Capture session already exists: {0}")]
    SessionAlreadyExists(String),

    /// An internal error occurred during the capture process.
    #[error("Capture error: {0}")]
    CaptureError(String),

    /// The current platform is not supported.
    #[error("Platform not supported: WASAPI is Windows-only")]
    Unsupported,

    /// Mobile-specific plugin invocation error.
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
