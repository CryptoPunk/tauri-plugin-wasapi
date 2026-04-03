use serde::{Deserialize, Serialize};

/// Audio device info returned to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioDevice {
    /// WASAPI device ID.
    pub id: String,
    /// Human-readable device name.
    pub name: String,
    /// "capture" or "render".
    pub direction: String,
    /// "active", "disabled", "notpresent", or "unplugged".
    pub state: String,
}

/// OS process info returned by list_processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessInfo {
    /// Process ID.
    pub pid: u32,
    /// Process name (executable basename).
    pub name: String,
    /// Parent Process ID.
    pub parent_pid: Option<u32>,
}

/// Request to start an audio capture session.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartCaptureRequest {
    /// Unique session identifier chosen by the caller.
    pub session_id: String,
    /// WASAPI device ID. None = system default.
    pub device_id: Option<String>,
    /// When true, capture output audio via loopback instead of mic input.
    #[serde(default)]
    pub loopback: bool,
    /// For application-specific capture (Win10 20348+). Overrides device_id.
    pub process_id: Option<u32>,
    /// Sample rate in Hz. Default: 48000.
    pub sample_rate: Option<u32>,
    /// Channel count. Default: 2 (stereo).
    pub channels: Option<u16>,
}

/// Request to stop a capture session.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopCaptureRequest {
    /// Session to stop.
    pub session_id: String,
}

/// Audio format metadata sent as the first channel message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioFormatInfo {
    pub session_id: String,
    pub sample_rate: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
    /// Always "f32" for this plugin.
    pub sample_format: String,
}

/// A chunk of captured PCM audio data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioChunk {
    pub session_id: String,
    /// Raw PCM bytes (f32le, interleaved channels).
    pub data: Vec<u8>,
    pub sample_rate: u32,
    pub channels: u16,
    /// Number of audio frames in this chunk.
    pub frames: u32,
}

/// Tagged stream event sent over the Tauri Channel IPC.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", content = "data", rename_all = "camelCase")]
pub enum StreamEvent {
    /// Initial format metadata.
    Format(AudioFormatInfo),
    /// Audio sample data.
    Data(AudioChunk),
    /// Non-fatal or fatal error during capture.
    Error {
        session_id: String,
        message: String,
    },
    /// Capture has stopped cleanly.
    Stopped {
        session_id: String,
    },
}
