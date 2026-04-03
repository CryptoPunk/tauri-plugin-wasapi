const COMMANDS: &[&str] = &[
    "list_devices",
    "list_processes",
    "start_capture",
    "stop_capture",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
