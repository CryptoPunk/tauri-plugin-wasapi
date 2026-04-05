/**
 * WASAPI audio capture plugin for Tauri 2.0.
 *
 * This module provides the frontend API for enumerating audio devices,
 * listing processes, and starting/stopping real-time audio capture.
 *
 * @module
 */

import { invoke, Channel } from '@tauri-apps/api/core';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/**
 * Audio device information from WASAPI enumeration.
 */
export interface AudioDevice {
  /** The unique WASAPI device identifier. */
  id: string;
  /** The human-readable name of the device. */
  name: string;
  /** The device direction: 'capture' (input) or 'render' (output). */
  direction: 'capture' | 'render';
  /** The current operational state (e.g., 'active', 'disabled'). */
  state: string;
}

/**
 * OS process information for application-specific capture.
 */
export interface ProcessInfo {
  /** The OS-assigned Process ID. */
  pid: number;
  /** The executable name (e.g., 'chrome.exe'). */
  name: string;
  /** The Parent Process ID, if available. */
  parentPid?: number;
}

/**
 * Audio format metadata sent as the first stream event.
 */
export interface AudioFormatInfo {
  /** The session identifier. */
  sessionId: string;
  /** The negotiated sample rate in Hz. */
  sampleRate: number;
  /** The negotiated number of channels. */
  channels: number;
  /** The bit depth per sample. Always 32 for this plugin. */
  bitsPerSample: number;
  /** The sample representation. Always 'f32' (32-bit float). */
  sampleFormat: string;
}

/**
 * A single chunk of raw PCM audio data.
 */
export interface AudioChunk {
  /** The session identifier. */
  sessionId: string;
  /**
   * Raw PCM bytes (f32le, interleaved) as a number array.
   * To use this as a Float32Array: `new Float32Array(new Uint8Array(chunk.data).buffer)`
   */
  data: number[];
  /** The sample rate of this data. */
  sampleRate: number;
  /** The number of channels in the interleaved data. */
  channels: number;
  /** The number of audio frames contained in this chunk. */
  frames: number;
}

/**
 * Tagged stream events received over the Tauri Channel IPC.
 */
export type StreamEvent =
  | {
      /** Initial format metadata sent at the start of a session. */
      event: 'format';
      data: AudioFormatInfo;
    }
  | {
      /** A chunk of captured audio data. */
      event: 'data';
      data: AudioChunk;
    }
  | {
      /** Sent if an error occurs during capture. */
      event: 'error';
      data: { sessionId: string; message: string };
    }
  | {
      /** Sent when the capture session has stopped successfully. */
      event: 'stopped';
      data: { sessionId: string };
    };

/**
 * Options for starting an audio capture session.
 */
export interface StartCaptureOptions {
  /** A unique session identifier chosen by the caller. */
  sessionId: string;
  /** The WASAPI device ID to capture from. Omit to use the system default. */
  deviceId?: string;
  /** When true, captures playback (output) audio via loopback instead of input. */
  loopback?: boolean;
  /** The Process ID for application-specific capture (Windows 10 20348+). */
  processId?: number;
  /** The desired sample rate in Hz (default: 48000). */
  sampleRate?: number;
  /** The desired channel count (default: 2). */
  channels?: number;
}

// ---------------------------------------------------------------------------
// API Functions
// ---------------------------------------------------------------------------

/**
 * Lists all available audio capture and render devices.
 *
 * @returns A promise resolving to an array of {@link AudioDevice} objects.
 *
 * @example
 * ```typescript
 * const devices = await listDevices();
 * const mics = devices.filter(d => d.direction === 'capture' && d.state === 'active');
 * ```
 */
export async function listDevices(): Promise<AudioDevice[]> {
  return await invoke<AudioDevice[]>('plugin:wasapi|list_devices');
}

/**
 * Lists running OS processes to facilitate application-specific capture.
 *
 * @returns A promise resolving to an array of {@link ProcessInfo} objects.
 *
 * @example
 * ```typescript
 * const procs = await listProcesses();
 * const spotify = procs.find(p => p.name.toLowerCase().includes('spotify'));
 * ```
 */
export async function listProcesses(): Promise<ProcessInfo[]> {
  return await invoke<ProcessInfo[]>('plugin:wasapi|list_processes');
}

/**
 * Starts an audio capture session.
 *
 * Audio data is streamed to the `onEvent` callback in real-time.
 * The first event will always be a `format` event describing the
 * audio format, followed by `data` events containing PCM chunks.
 *
 * @param options  Configuration for the capture session.
 * @param onEvent  Callback invoked for each stream event.
 * @returns A promise that resolves once the session has started.
 *
 * @example
 * ```typescript
 * await startCapture(
 *   { sessionId: 'main-mic', loopback: false },
 *   (event) => {
 *     if (event.event === 'data') {
 *       console.log(`Received ${event.data.frames} frames`);
 *     }
 *   }
 * );
 * ```
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
 * Stops a running capture session.
 *
 * @param sessionId  The session to stop (must match a prior `startCapture` call).
 * @returns A promise that resolves once the stop request has been processed.
 *
 * @example
 * ```typescript
 * await stopCapture('main-mic');
 * ```
 */
export async function stopCapture(sessionId: string): Promise<void> {
  await invoke('plugin:wasapi|stop_capture', {
    request: { sessionId },
  });
}
