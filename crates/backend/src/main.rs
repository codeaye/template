#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn hello(name: &str) -> Result<String, String> {
    if name.contains(' ') {
        Err("Name should not contain spaces".to_string())
    } else {
        Ok(format!("Hello, {}", name))
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![hello])
        .run(tauri::generate_context!())
        .expect("There was an error while running tauri application");
}
