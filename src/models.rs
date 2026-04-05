use serde::{Deserialize, Serialize};

/// Audio device information returned to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioDevice {
    /// The unique WASAPI device identifier.
    pub id: String,
    /// The human-readable name of the device (e.g., "Microphone (Realtek Audio)").
    pub name: String,
    /// The device direction: "capture" (input) or "render" (output).
    pub direction: String,
    /// The current operational state: "active", "disabled", "notpresent", or "unplugged".
    pub state: String,
}

/// OS process information returned by `list_processes`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessInfo {
    /// The OS-assigned Process ID.
    pub pid: u32,
    /// The executable name (e.g., "chrome.exe").
    pub name: String,
    /// The Parent Process ID, if available.
    pub parent_pid: Option<u32>,
}

/// Configuration for starting a new audio capture session.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartCaptureRequest {
    /// A unique session identifier chosen by the caller. Used to stop the session.
    pub session_id: String,
    /// The WASAPI device ID to capture from. If `None`, uses the system default.
    pub device_id: Option<String>,
    /// When true, captures playback (output) audio via loopback instead of input.
    #[serde(default)]
    pub loopback: bool,
    /// For application-specific capture (Windows 10 20348+). Overrides `device_id`.
    pub process_id: Option<u32>,
    /// The desired sample rate in Hz. Default: 48000.
    pub sample_rate: Option<u32>,
    /// The desired number of audio channels. Default: 2 (stereo).
    pub channels: Option<u16>,
}

/// Request to stop an existing capture session.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopCaptureRequest {
    /// The unique identifier for the session to stop.
    pub session_id: String,
}

/// Audio format metadata sent as the first message on a capture stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioFormatInfo {
    /// The session identifier.
    pub session_id: String,
    /// The negotiated sample rate in Hz.
    pub sample_rate: u32,
    /// The negotiated number of channels.
    pub channels: u16,
    /// The bit depth per sample. Always 32 for this plugin.
    pub bits_per_sample: u16,
    /// The sample representation. Always "f32" (32-bit float).
    pub sample_format: String,
}

/// A single chunk of captured PCM audio data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioChunk {
    /// The session identifier.
    pub session_id: String,
    /// The raw PCM bytes. These are 32-bit floats ("f32le") with interleaved channels.
    pub data: Vec<u8>,
    /// The sample rate of this data.
    pub sample_rate: u32,
    /// The number of channels in the interleaved data.
    pub channels: u16,
    /// The number of audio frames contained in this chunk.
    pub frames: u32,
}

/// Tagged stream events sent over the Tauri Channel IPC.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", content = "data", rename_all = "camelCase")]
pub enum StreamEvent {
    /// The first event sent, describing the negotiated audio format.
    Format(AudioFormatInfo),
    /// A regular event containing a chunk of captured audio data.
    Data(AudioChunk),
    /// Sent if a non-fatal or fatal error occurs during capture.
    Error {
        /// The session identifier.
        session_id: String,
        /// A human-readable error message.
        message: String,
    },
    /// Sent when the capture session has stopped successfully.
    Stopped {
        /// The session identifier.
        session_id: String,
    },
}
