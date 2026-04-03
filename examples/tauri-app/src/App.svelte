<script>
  import {
    listDevices,
    listProcesses,
    startCapture,
    stopCapture,
  } from 'tauri-plugin-wasapi-api';

  let devices = $state([]);
  let processes = $state([]);
  let logs = $state([]);
  let capturing = $state(false);
  let sessionId = $state('demo-session');
  let selectedDeviceId = $state('');
  let loopback = $state(false);
  let selectedPid = $state('');
  let sampleRate = $state(48000);
  let channels = $state(2);
  let chunkCount = $state(0);
  let totalFrames = $state(0);
  let formatInfo = $state(null);

  function log(msg) {
    const ts = new Date().toLocaleTimeString();
    logs = [...logs.slice(-99), `[${ts}] ${msg}`];
  }

  async function handleListDevices() {
    try {
      devices = await listDevices();
      log(`Found ${devices.length} devices`);
    } catch (e) {
      log(`Error listing devices: ${e}`);
    }
  }

  async function handleListProcesses() {
    try {
      processes = await listProcesses();
      log(`Found ${processes.length} processes`);
    } catch (e) {
      log(`Error listing processes: ${e}`);
    }
  }

  async function handleStartCapture() {
    chunkCount = 0;
    totalFrames = 0;
    formatInfo = null;

    const opts = {
      sessionId,
      loopback,
      sampleRate,
      channels,
    };

    if (selectedDeviceId) {
      opts.deviceId = selectedDeviceId;
    }

    if (selectedPid) {
      opts.processId = parseInt(selectedPid, 10);
    }

    try {
      log(`Starting capture: ${JSON.stringify(opts)}`);
      capturing = true;

      await startCapture(opts, (event) => {
        switch (event.event) {
          case 'format':
            formatInfo = event.data;
            log(
              `Format: ${event.data.sampleRate}Hz, ${event.data.channels}ch, ${event.data.bitsPerSample}bit ${event.data.sampleFormat}`,
            );
            break;
          case 'data':
            chunkCount++;
            totalFrames += event.data.frames;
            break;
          case 'error':
            log(`Capture error: ${event.data.message}`);
            capturing = false;
            break;
          case 'stopped':
            log('Capture stopped');
            capturing = false;
            break;
        }
      });

      log('Capture command accepted');
    } catch (e) {
      log(`Error starting capture: ${e}`);
      capturing = false;
    }
  }

  async function handleStopCapture() {
    try {
      log('Stopping capture...');
      await stopCapture(sessionId);
    } catch (e) {
      log(`Error stopping capture: ${e}`);
    }
  }
</script>

<main class="container">
  <h1>WASAPI Audio Capture</h1>

  <section class="panel">
    <h2>Devices</h2>
    <button id="btn-list-devices" onclick={handleListDevices}>
      List Devices
    </button>

    {#if devices.length > 0}
      <select id="device-select" bind:value={selectedDeviceId}>
        <option value="">System Default</option>
        {#each devices as device}
          <option value={device.id}>
            {device.name} ({device.direction}) — {device.state}
          </option>
        {/each}
      </select>
    {/if}
  </section>

  <section class="panel">
    <h2>Processes</h2>
    <button id="btn-list-processes" onclick={handleListProcesses}>
      List Processes
    </button>

    {#if processes.length > 0}
      <select id="process-select" bind:value={selectedPid}>
        <option value="">None (device capture)</option>
        {#each processes as proc}
          <option value={proc.pid}>
            {proc.name} (PID {proc.pid})
          </option>
        {/each}
      </select>
    {/if}
  </section>

  <section class="panel">
    <h2>Capture Settings</h2>
    <div class="settings-grid">
      <label for="session-id">Session ID</label>
      <input id="session-id" type="text" bind:value={sessionId} />

      <label for="sample-rate">Sample Rate</label>
      <select id="sample-rate" bind:value={sampleRate}>
        <option value={44100}>44100 Hz</option>
        <option value={48000}>48000 Hz</option>
        <option value={96000}>96000 Hz</option>
      </select>

      <label for="channels">Channels</label>
      <select id="channels" bind:value={channels}>
        <option value={1}>Mono</option>
        <option value={2}>Stereo</option>
      </select>

      <label for="loopback">Loopback</label>
      <input id="loopback" type="checkbox" bind:checked={loopback} />
    </div>

    <div class="capture-buttons">
      {#if !capturing}
        <button id="btn-start-capture" onclick={handleStartCapture}>
          ▶ Start Capture
        </button>
      {:else}
        <button id="btn-stop-capture" class="stop" onclick={handleStopCapture}>
          ■ Stop Capture
        </button>
      {/if}
    </div>
  </section>

  {#if capturing || formatInfo}
    <section class="panel stats">
      <h2>Stream Info</h2>
      {#if formatInfo}
        <p>
          <strong>Format:</strong>
          {formatInfo.sampleRate}Hz / {formatInfo.channels}ch /
          {formatInfo.bitsPerSample}bit {formatInfo.sampleFormat}
        </p>
      {/if}
      <p><strong>Chunks received:</strong> {chunkCount}</p>
      <p><strong>Total frames:</strong> {totalFrames.toLocaleString()}</p>
      {#if formatInfo && totalFrames > 0}
        <p>
          <strong>Duration:</strong>
          {(totalFrames / formatInfo.sampleRate).toFixed(1)}s
        </p>
      {/if}
    </section>
  {/if}

  <section class="panel log-panel">
    <h2>Log</h2>
    <div class="log">
      {#each logs as entry}
        <div class="log-entry">{entry}</div>
      {/each}
    </div>
  </section>
</main>

<style>
  .container {
    max-width: 700px;
    margin: 0 auto;
    padding: 1rem;
  }

  h1 {
    text-align: center;
    margin-bottom: 1.5rem;
  }

  .panel {
    background: var(--panel-bg, rgba(255, 255, 255, 0.05));
    border: 1px solid var(--border, rgba(255, 255, 255, 0.1));
    border-radius: 8px;
    padding: 1rem;
    margin-bottom: 1rem;
  }

  .panel h2 {
    margin: 0 0 0.75rem;
    font-size: 1rem;
    opacity: 0.7;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  select {
    width: 100%;
    padding: 0.5em;
    margin-top: 0.5em;
    border-radius: 6px;
    border: 1px solid var(--border, rgba(255, 255, 255, 0.15));
    background: var(--input-bg, rgba(0, 0, 0, 0.2));
    color: inherit;
    font-family: inherit;
    font-size: 0.9em;
  }

  .settings-grid {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 0.5rem 1rem;
    align-items: center;
  }

  .capture-buttons {
    margin-top: 1rem;
    text-align: center;
  }

  button.stop {
    background-color: #c0392b;
    border-color: #e74c3c;
  }

  button.stop:hover {
    background-color: #e74c3c;
  }

  .stats p {
    margin: 0.25rem 0;
  }

  .log-panel {
    max-height: 200px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .log {
    flex: 1;
    overflow-y: auto;
    font-family: monospace;
    font-size: 0.8em;
    line-height: 1.4;
    max-height: 160px;
  }

  .log-entry {
    padding: 0.1rem 0;
    opacity: 0.85;
  }
</style>
