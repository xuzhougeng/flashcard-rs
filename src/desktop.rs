#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Manager};
use tauri_plugin_window_state::WindowExt;
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_store::StoreExt;
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
    // Update settings in memory
    {
        let mut app_settings = state.settings.lock().map_err(|e| e.to_string())?;
        *app_settings = settings.clone();
    }

    // Persist settings to disk
    let store = app.store("settings.json")
        .map_err(|e| format!("Failed to get store: {}", e))?;

    store.set("interval", serde_json::json!(settings.interval));
    store.set("autostart", serde_json::json!(settings.autostart));
    store.set("card_type", serde_json::json!(settings.card_type.clone()));

    store.save()
        .map_err(|e| format!("Failed to save settings: {}", e))?;

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
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(move |app| {
            let app_handle = app.handle().clone();

            // Load settings from persistent storage
            let store = app.store("settings.json")
                .expect("Failed to initialize store");

            let settings = Settings {
                interval: store.get("interval")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(10),
                autostart: store.get("autostart")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
                card_type: store.get("card_type")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "mixed".to_string()),
            };

            let state = AppState {
                settings: Arc::new(Mutex::new(settings)),
                timer_handle: Arc::new(Mutex::new(None)),
            };

            app.manage(state.clone());

            // Restore window state for the main window created from tauri.conf.json
            if let Some(win) = app.get_webview_window("main") {
                let _ = win.restore_state(Default::default());
            }

            // Start timer
            restart_timer(&app_handle, &state);

            // Setup cleanup handler for app exit
            let cleanup_state = state.clone();
            app.on_window_event(move |window, event| {
                if let tauri::WindowEvent::CloseRequested { .. } = event {
                    // Abort the timer task on window close
                    if let Ok(mut timer_handle) = cleanup_state.timer_handle.lock() {
                        if let Some(handle) = timer_handle.take() {
                            handle.abort();
                        }
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_settings, save_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
