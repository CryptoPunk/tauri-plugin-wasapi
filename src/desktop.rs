use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use tauri::ipc::Channel;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

/// Tracks a running capture session.
struct CaptureSession {
    /// Set to true to signal the capture thread to stop.
    stop_flag: Arc<AtomicBool>,
    /// Handle to the capture thread.
    handle: Option<JoinHandle<()>>,
}

/// Plugin state managed by Tauri.
pub struct Wasapi<R: Runtime> {
    #[allow(dead_code)]
    app: AppHandle<R>,
    sessions: Arc<Mutex<HashMap<String, CaptureSession>>>,
}

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Wasapi<R>> {
    Ok(Wasapi {
        app: app.clone(),
        sessions: Arc::new(Mutex::new(HashMap::new())),
    })
}

impl<R: Runtime> Wasapi<R> {
    /// Enumerate all audio capture and render devices.
    pub fn list_devices(&self) -> crate::Result<Vec<AudioDevice>> {
        #[cfg(windows)]
        {
            list_devices_impl()
        }
        #[cfg(not(windows))]
        {
            Err(crate::Error::Unsupported)
        }
    }

    /// List OS processes (for application-specific capture).
    pub fn list_processes(&self) -> crate::Result<Vec<ProcessInfo>> {
        #[cfg(windows)]
        {
            list_processes_impl()
        }
        #[cfg(not(windows))]
        {
            Err(crate::Error::Unsupported)
        }
    }

    /// Start a capture session on a dedicated thread, streaming audio
    /// data to the frontend via the provided Channel.
    pub fn start_capture(
        &self,
        request: StartCaptureRequest,
        on_event: Channel<StreamEvent>,
    ) -> crate::Result<()> {
        let session_id = request.session_id.clone();
        let session_id_for_insert = session_id.clone();

        let mut sessions = self
            .sessions
            .lock()
            .map_err(|e| crate::Error::CaptureError(e.to_string()))?;

        if sessions.contains_key(&session_id) {
            return Err(crate::Error::SessionAlreadyExists(session_id));
        }

        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_flag_clone = stop_flag.clone();
        let sessions_map = self.sessions.clone();

        let handle = thread::Builder::new()
            .name(format!("wasapi-capture-{}", &session_id))
            .spawn(move || {
                #[cfg(windows)]
                {
                    let result =
                        capture_thread(request, on_event.clone(), stop_flag_clone);
                    if let Err(e) = result {
                        let _ = on_event.send(StreamEvent::Error {
                            session_id: session_id.clone(),
                            message: e.to_string(),
                        });
                    }
                }
                #[cfg(not(windows))]
                {
                    let _ = on_event.send(StreamEvent::Error {
                        session_id: session_id.clone(),
                        message: "WASAPI is only supported on Windows".to_string(),
                    });
                    let _ = stop_flag_clone; // suppress unused warning
                }

                // Remove self from the sessions map on exit.
                if let Ok(mut map) = sessions_map.lock() {
                    map.remove(&session_id);
                }
            })
            .map_err(|e| crate::Error::CaptureError(e.to_string()))?;

        sessions.insert(
            session_id_for_insert,
            CaptureSession {
                stop_flag,
                handle: Some(handle),
            },
        );

        Ok(())
    }

    /// Signal a capture session to stop and wait for its thread to finish.
    pub fn stop_capture(&self, session_id: &str) -> crate::Result<()> {
        let mut sessions = self
            .sessions
            .lock()
            .map_err(|e| crate::Error::CaptureError(e.to_string()))?;

        let session = sessions
            .remove(session_id)
            .ok_or_else(|| crate::Error::SessionNotFound(session_id.to_string()))?;

        // Signal the capture thread to stop.
        session.stop_flag.store(true, Ordering::SeqCst);

        // Drop the lock before joining to avoid deadlock — the thread
        // also tries to lock sessions on exit.
        drop(sessions);

        if let Some(handle) = session.handle {
            let _ = handle.join();
        }

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Windows-only implementations
// ---------------------------------------------------------------------------

#[cfg(windows)]
fn list_devices_impl() -> crate::Result<Vec<AudioDevice>> {
    use wasapi::*;

    initialize_mta()
        .ok()
        .map_err(|e| crate::Error::Wasapi(e.to_string()))?;

    let enumerator =
        DeviceEnumerator::new().map_err(|e| crate::Error::Wasapi(e.to_string()))?;

    let mut devices = Vec::new();

    for (dir_variant, dir_name) in [
        (Direction::Capture, "capture"),
        (Direction::Render, "render"),
    ] {
        let collection = enumerator
            .get_device_collection(&dir_variant)
            .map_err(|e| crate::Error::Wasapi(e.to_string()))?;

        for device_result in &collection {
            let device = device_result.map_err(|e| crate::Error::Wasapi(e.to_string()))?;
            let name = device
                .get_friendlyname()
                .unwrap_or_else(|_| "<unknown>".to_string());
            let id = device
                .get_id()
                .map_err(|e| crate::Error::Wasapi(e.to_string()))?;
            let state = device
                .get_state()
                .map(|s| format!("{:?}", s).to_lowercase())
                .unwrap_or_else(|_| "unknown".to_string());

            devices.push(AudioDevice {
                id,
                name,
                direction: dir_name.to_string(),
                state,
            });
        }
    }

    Ok(devices)
}

#[cfg(windows)]
fn list_processes_impl() -> crate::Result<Vec<ProcessInfo>> {
    use sysinfo::{ProcessRefreshKind, RefreshKind, System};

    let refreshes =
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything());
    let system = System::new_with_specifics(refreshes);

    let mut processes: Vec<ProcessInfo> = system
        .processes()
        .iter()
        .map(|(pid, proc_info)| ProcessInfo {
            pid: pid.as_u32(),
            name: proc_info.name().to_string_lossy().to_string(),
            parent_pid: proc_info.parent().map(|p| p.as_u32()),
        })
        .collect();

    // Sort by name for convenience.
    processes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(processes)
}

/// The main capture loop running on a dedicated thread.
///
/// Initialises COM, opens the requested WASAPI device, sends format info,
/// then streams PCM chunks over the channel until stopped.
#[cfg(windows)]
fn capture_thread(
    request: StartCaptureRequest,
    on_event: Channel<StreamEvent>,
    stop_flag: Arc<AtomicBool>,
) -> crate::Result<()> {
    use std::collections::VecDeque;
    use wasapi::*;

    let session_id = request.session_id.clone();
    let sample_rate = request.sample_rate.unwrap_or(48000);
    let channels = request.channels.unwrap_or(2);
    let bits_per_sample: u16 = 32;
    let chunk_frames: usize = 4096;

    initialize_mta()
        .ok()
        .map_err(|e| crate::Error::Wasapi(e.to_string()))?;

    let desired_format = WaveFormat::new(
        bits_per_sample as usize,
        bits_per_sample as usize,
        &SampleType::Float,
        sample_rate as usize,
        channels as usize,
        None,
    );
    let blockalign = desired_format.get_blockalign();

    // Open the appropriate AudioClient based on capture mode.
    let mut audio_client = if let Some(pid) = request.process_id {
        // Application-specific loopback capture.
        AudioClient::new_application_loopback_client(pid, true)
            .map_err(|e| crate::Error::Wasapi(e.to_string()))?
    } else {
        // Device capture (mic or loopback).
        let enumerator =
            DeviceEnumerator::new().map_err(|e| crate::Error::Wasapi(e.to_string()))?;

        let direction = if request.loopback {
            Direction::Render
        } else {
            Direction::Capture
        };

        let device = if let Some(ref id) = request.device_id {
            enumerator
                .get_device(id)
                .map_err(|e| crate::Error::Wasapi(e.to_string()))?
        } else {
            enumerator
                .get_default_device(&direction)
                .map_err(|e| crate::Error::Wasapi(e.to_string()))?
        };

        device
            .get_iaudioclient()
            .map_err(|e| crate::Error::Wasapi(e.to_string()))?
    };

    // Determine the capture direction for initialization.
    let init_direction = if request.process_id.is_some() {
        // Application loopback always initialises as Capture.
        Direction::Capture
    } else if request.loopback {
        Direction::Render
    } else {
        Direction::Capture
    };

    // get_device_period() is not supported on application loopback virtual
    // devices (returns E_NOTIMPL), so we skip it and use 0 — the buffer
    // duration is ignored by WASAPI in that mode anyway.
    let buffer_duration_hns = if request.process_id.is_some() {
        0
    } else {
        let (_def_time, min_time) = audio_client
            .get_device_period()
            .map_err(|e| crate::Error::Wasapi(e.to_string()))?;
        min_time
    };

    let mode = StreamMode::EventsShared {
        autoconvert: true,
        buffer_duration_hns,
    };

    audio_client
        .initialize_client(&desired_format, &init_direction, &mode)
        .map_err(|e| crate::Error::Wasapi(e.to_string()))?;

    let h_event = audio_client
        .set_get_eventhandle()
        .map_err(|e| crate::Error::Wasapi(e.to_string()))?;

    let capture_client = audio_client
        .get_audiocaptureclient()
        .map_err(|e| crate::Error::Wasapi(e.to_string()))?;

    // Send format info as the first event.
    let _ = on_event.send(StreamEvent::Format(AudioFormatInfo {
        session_id: session_id.clone(),
        sample_rate,
        channels,
        bits_per_sample,
        sample_format: "f32".to_string(),
    }));

    let mut sample_queue: VecDeque<u8> = VecDeque::new();
    let chunk_bytes = blockalign as usize * chunk_frames;

    audio_client
        .start_stream()
        .map_err(|e| crate::Error::Wasapi(e.to_string()))?;

    log::debug!("[wasapi] capture started: session={}", session_id);

    loop {
        // Check stop flag.
        if stop_flag.load(Ordering::SeqCst) {
            log::debug!("[wasapi] stop flag set for session={}", session_id);
            break;
        }

        // Drain accumulated samples into chunk-sized messages.
        while sample_queue.len() >= chunk_bytes {
            let mut chunk = vec![0u8; chunk_bytes];
            for byte in chunk.iter_mut() {
                *byte = sample_queue.pop_front().unwrap();
            }
            let _ = on_event.send(StreamEvent::Data(AudioChunk {
                session_id: session_id.clone(),
                data: chunk,
                sample_rate,
                channels,
                frames: chunk_frames as u32,
            }));
        }

        // Read new samples from the device.
        if let Err(e) =
            capture_client.read_from_device_to_deque(&mut sample_queue)
        {
            log::error!("[wasapi] read error: {}", e);
            let _ = on_event.send(StreamEvent::Error {
                session_id: session_id.clone(),
                message: format!("Read error: {}", e),
            });
            break;
        }

        // Wait for the next buffer event (timeout 3 s).
        if h_event.wait_for_event(3000).is_err() {
            log::warn!("[wasapi] event timeout for session={}", session_id);
            // Timeout is not necessarily fatal — check stop flag and retry.
            if stop_flag.load(Ordering::SeqCst) {
                break;
            }
        }
    }

    // Flush remaining samples as a final partial chunk.
    if !sample_queue.is_empty() {
        let remaining: Vec<u8> = sample_queue.drain(..).collect();
        let remaining_frames =
            remaining.len() as u32 / blockalign as u32;
        if remaining_frames > 0 {
            let _ = on_event.send(StreamEvent::Data(AudioChunk {
                session_id: session_id.clone(),
                data: remaining,
                sample_rate,
                channels,
                frames: remaining_frames,
            }));
        }
    }

    let _ = audio_client.stop_stream();

    let _ = on_event.send(StreamEvent::Stopped {
        session_id: session_id.clone(),
    });

    log::debug!("[wasapi] capture stopped: session={}", session_id);
    Ok(())
}
