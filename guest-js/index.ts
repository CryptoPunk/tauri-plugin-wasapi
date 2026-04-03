import { invoke, Channel } from '@tauri-apps/api/core';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/** Audio device info from WASAPI enumeration. */
export interface AudioDevice {
  id: string;
  name: string;
  direction: 'capture' | 'render';
  state: string;
}

/** OS process info for application-specific capture. */
export interface ProcessInfo {
  pid: number;
  name: string;
}

/** Audio format metadata sent as the first stream event. */
export interface AudioFormatInfo {
  sessionId: string;
  sampleRate: number;
  channels: number;
  bitsPerSample: number;
  sampleFormat: string;
}

/** A chunk of raw PCM audio data. */
export interface AudioChunk {
  sessionId: string;
  /** Raw PCM bytes (f32le, interleaved) as a number array. */
  data: number[];
  sampleRate: number;
  channels: number;
  /** Number of audio frames in this chunk. */
  frames: number;
}

/** Tagged stream events received over the Tauri Channel IPC. */
export type StreamEvent =
  | { event: 'format'; data: AudioFormatInfo }
  | { event: 'data'; data: AudioChunk }
  | { event: 'error'; data: { sessionId: string; message: string } }
  | { event: 'stopped'; data: { sessionId: string } };

/** Options for starting an audio capture session. */
export interface StartCaptureOptions {
  /** Unique session identifier. */
  sessionId: string;
  /** WASAPI device ID. Omit to use system default. */
  deviceId?: string;
  /** Capture output audio via loopback instead of mic input. */
  loopback?: boolean;
  /** Process ID for application-specific capture (Win10 20348+). */
  processId?: number;
  /** Sample rate in Hz (default: 48000). */
  sampleRate?: number;
  /** Channel count (default: 2). */
  channels?: number;
}

// ---------------------------------------------------------------------------
// API Functions
// ---------------------------------------------------------------------------

/**
 * List all available audio capture and render devices.
 */
export async function listDevices(): Promise<AudioDevice[]> {
  return await invoke<AudioDevice[]>('plugin:wasapi|list_devices');
}

/**
 * List running OS processes (for application-specific capture).
 */
export async function listProcesses(): Promise<ProcessInfo[]> {
  return await invoke<ProcessInfo[]>('plugin:wasapi|list_processes');
}

/**
 * Start an audio capture session.
 *
 * Audio data is streamed to the `onEvent` callback in real-time.
 * The first event will always be a `format` event describing the
 * audio format, followed by `data` events containing PCM chunks.
 *
 * @param options  Capture configuration.
 * @param onEvent  Callback invoked for each stream event.
 */
export async function startCapture(
  options: StartCaptureOptions,
  onEvent: (event: StreamEvent) => void,
): Promise<void> {
  const channel = new Channel<StreamEvent>();
  channel.onmessage = onEvent;

  await invoke('plugin:wasapi|start_capture', {
    request: options,
    onEvent: channel,
  });
}

/**
 * Stop a running capture session.
 *
 * @param sessionId  The session to stop (must match a prior startCapture call).
 */
export async function stopCapture(sessionId: string): Promise<void> {
  await invoke('plugin:wasapi|stop_capture', {
    request: { sessionId },
  });
}
