# Permissions

This document describes the Tauri 2 permission system for the `tauri-plugin-wasapi` and how to configure it in your application.

## Overview

Tauri 2 introduces a powerful command-level permission system. By default, all commands in a plugin are **denied** unless explicitly allowed by a capability.

## Default Permissions

The plugin provides a predefined set of permissions in `permissions/default.toml`.

### `wasapi:default`
This permission set allows access to all commands in the plugin:
- `list_devices`
- `list_processes`
- `start_capture`
- `stop_capture`

## Configuring Capabilities

To use the plugin in your Tauri application, you must add the permission to your application's capability configuration (usually found in `src-tauri/capabilities/default.json`).

### Example: `src-tauri/capabilities/default.json`

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "wasapi:default"
  ]
}
```

## Custom Permissions

If you want to restrict access to certain commands, you can define your own capabilities that only include specific permissions.

### Permission Identifiers
| Permission | Description |
|---|---|
| `wasapi:allow-list-devices` | Allows listing audio endpoints. |
| `wasapi:allow-list-processes` | Allows listing OS processes. |
| `wasapi:allow-start-capture` | Allows starting a capture session. |
| `wasapi:allow-stop-capture` | Allows stopping a capture session. |

### Example: Read-Only Access
If you only want your application to be able to list devices but not capture audio:

```json
{
  "permissions": [
    "wasapi:allow-list-devices"
  ]
}
```

## Privacy and OS Permissions

> [!WARNING]
> While Tauri's permission system controls access to the plugin's commands, the **Windows OS** also has its own privacy settings for microphone access.

If your application has the `wasapi:default` permission but the user has disabled "Microphone access" in Windows settings, the `start_capture` command will still fail with an "Access Denied" error. You should handle this in your application's error logic.
