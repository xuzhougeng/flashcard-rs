#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Manager};
use tauri_plugin_window_state::WindowExt;
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_store::StoreExt;

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
    timer_running: Arc<Mutex<bool>>,
    timer_generation: Arc<Mutex<u64>>,
}

#[tauri::command]
fn get_settings(state: tauri::State<AppState>) -> Result<Settings, String> {
    let settings = state.settings.lock().map_err(|e| e.to_string())?;
    Ok(settings.clone())
}

#[tauri::command]
fn save_settings(
    settings: Settings,
    state: tauri::State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    // Update settings in memory
    {
        let mut app_settings = state.settings.lock().map_err(|e| format!("Failed to lock settings: {}", e))?;
        *app_settings = settings.clone();
    }

    // Persist settings to disk
    let store = app.store("settings.json")
        .map_err(|e| format!("Failed to get store: {}", e))?;

    // Set values in store
    store.set("interval", serde_json::json!(settings.interval));
    store.set("autostart", serde_json::json!(settings.autostart));
    store.set("card_type", serde_json::json!(settings.card_type.clone()));

    // Save store to disk
    store.save()
        .map_err(|e| format!("Failed to save store to disk: {}", e))?;

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

    // Increment generation counter to stop old timer tasks
    if let Ok(mut gen) = state.timer_generation.lock() {
        *gen = gen.wrapping_add(1);
    }

    // Set timer to running
    if let Ok(mut running) = state.timer_running.lock() {
        *running = true;
    }

    // Spawn new timer task with generation tracking
    tauri::async_runtime::spawn(async move {
        // Capture current generation
        let my_generation = {
            let gen = state_clone.timer_generation.lock().ok();
            gen.map(|g| *g).unwrap_or(0)
        };

        loop {
            // Check if this task should still be running
            let should_continue = {
                match state_clone.timer_running.lock() {
                    Ok(running) => *running,
                    Err(e) => {
                        eprintln!("Failed to lock timer_running mutex: {}", e);
                        break;
                    }
                }
            };

            // Check if this is still the current generation
            let is_current_generation = {
                match state_clone.timer_generation.lock() {
                    Ok(gen) => *gen == my_generation,
                    Err(e) => {
                        eprintln!("Failed to lock timer_generation mutex: {}", e);
                        break;
                    }
                }
            };

            if !should_continue || !is_current_generation {
                break;
            }

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

            // Verify again before showing window
            let is_still_current = {
                state_clone.timer_generation.lock()
                    .ok()
                    .map(|g| *g == my_generation)
                    .unwrap_or(false)
            };

            if !is_still_current {
                break;
            }

            // Show window
            if let Some(window) = app_handle.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
    });
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
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| "mixed".to_string()),
            };

            let state = AppState {
                settings: Arc::new(Mutex::new(settings)),
                timer_running: Arc::new(Mutex::new(true)),
                timer_generation: Arc::new(Mutex::new(0)),
            };

            app.manage(state.clone());

            // Restore window state for the main window created from tauri.conf.json
            if let Some(win) = app.get_webview_window("main") {
                let _ = win.restore_state(Default::default());

                // Setup cleanup handler for window close
                let cleanup_state = state.clone();
                win.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { .. } = event {
                        // Stop the timer task on window close
                        if let Ok(mut running) = cleanup_state.timer_running.lock() {
                            *running = false;
                        }
                    }
                });
            }

            // Start timer
            restart_timer(&app_handle, &state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_settings, save_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
