#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Manager};
use tauri_plugin_window_state::WindowExt;
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tokio::task::AbortHandle;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Settings {
    interval: u64,       // in minutes
    autostart: bool,
    card_type: String,   // "romaji", "chinese", or "mixed"
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            interval: 10,
            autostart: false,
            card_type: "mixed".to_string(),
        }
    }
}

#[derive(Clone)]
struct AppState {
    settings: Arc<Mutex<Settings>>,
    timer_handle: Arc<Mutex<Option<AbortHandle>>>,
}

#[tauri::command]
fn get_settings(state: tauri::State<AppState>) -> Result<Settings, String> {
    let settings = state.settings.lock().map_err(|e| e.to_string())?;
    Ok(settings.clone())
}

#[tauri::command]
async fn save_settings(
    settings: Settings,
    state: tauri::State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    // Update settings
    {
        let mut app_settings = state.settings.lock().map_err(|e| e.to_string())?;
        *app_settings = settings.clone();
    }

    // Handle autostart
    let autostart_manager = app.autolaunch();
    if settings.autostart {
        autostart_manager
            .enable()
            .map_err(|e| format!("Failed to enable autostart: {}", e))?;
    } else {
        autostart_manager
            .disable()
            .map_err(|e| format!("Failed to disable autostart: {}", e))?;
    }

    // Restart timer with new interval
    restart_timer(&app, &state);

    Ok(())
}

fn restart_timer(app: &AppHandle, state: &AppState) {
    let app_handle = app.clone();
    let state_clone = state.clone();

    // Cancel the old timer task if it exists
    if let Ok(mut timer_handle) = state.timer_handle.lock() {
        if let Some(handle) = timer_handle.take() {
            handle.abort();
        }
    }

    // Spawn new timer task and store its abort handle
    let task_handle = tauri::async_runtime::spawn(async move {
        loop {
            // Get current interval with proper error handling
            let interval = {
                match state_clone.settings.lock() {
                    Ok(settings) => settings.interval,
                    Err(e) => {
                        eprintln!("Failed to lock settings mutex: {}", e);
                        break;
                    }
                }
            };

            // Wait for interval
            tokio::time::sleep(Duration::from_secs(interval * 60)).await;

            // Show window
            if let Some(window) = app_handle.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
    });

    // Store the new task's abort handle
    if let Ok(mut timer_handle) = state.timer_handle.lock() {
        *timer_handle = Some(task_handle.abort_handle());
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    let state = AppState {
        settings: Arc::new(Mutex::new(Settings::default())),
        timer_handle: Arc::new(Mutex::new(None)),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .manage(state.clone())
        .invoke_handler(tauri::generate_handler![get_settings, save_settings])
        .setup(move |app| {
            let app_handle = app.handle().clone();

            // Restore window state for the main window created from tauri.conf.json
            if let Some(win) = app.get_webview_window("main") {
                let _ = win.restore_state(Default::default());
            }

            // Start timer
            restart_timer(&app_handle, &state);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
