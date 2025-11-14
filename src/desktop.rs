#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Manager};
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri_plugin_window_state::WindowExt;
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_store::StoreExt;
use tauri_plugin_notification::NotificationExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Settings {
    interval: u64,       // in seconds
    autostart: bool,
    card_type: String,   // "romaji", "chinese", or "mixed"
    close_behavior: Option<String>, // "minimize" or "exit", None means ask every time
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            interval: 600,  // 10 minutes in seconds
            autostart: false,
            card_type: "mixed".to_string(),
            close_behavior: Some("minimize".to_string()),  // Default to minimize to tray
        }
    }
}

#[derive(Clone)]
struct AppState {
    settings: Arc<Mutex<Settings>>,
    timer_running: Arc<Mutex<bool>>,
    timer_generation: Arc<Mutex<u64>>,
    window_hidden: Arc<Mutex<bool>>, // Track if window is hidden to tray
}

// Helper function to handle close action
async fn handle_close_action(behavior: &str, state: &AppState, app: &AppHandle) {
    match behavior {
        "minimize" => {
            // Hide window and mark as hidden
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
                if let Ok(mut hidden) = state.window_hidden.lock() {
                    *hidden = true;
                }
            }
        }
        "exit" => {
            // Stop the timer task and exit
            if let Ok(mut running) = state.timer_running.lock() {
                *running = false;
            }
            std::process::exit(0);
        }
        _ => {
            eprintln!("Unknown close behavior: {}", behavior);
        }
    }
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
    store.set("close_behavior", serde_json::json!(settings.close_behavior.clone()));

    // Save store to disk
    store.save()
        .map_err(|e| format!("Failed to save store to disk: {}", e))?;

    // Handle autostart (non-fatal - log errors but don't fail settings save)
    let autostart_manager = app.autolaunch();
    if settings.autostart {
        if let Err(e) = autostart_manager.enable() {
            eprintln!("Warning: Failed to enable autostart: {}", e);
        }
    } else {
        // Only try to disable if it's actually enabled
        if let Ok(is_enabled) = autostart_manager.is_enabled() {
            if is_enabled {
                if let Err(e) = autostart_manager.disable() {
                    eprintln!("Warning: Failed to disable autostart: {}", e);
                }
            }
        }
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

            // Wait for interval (interval is now in seconds)
            tokio::time::sleep(Duration::from_secs(interval)).await;

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

            // Check if window is hidden to tray
            let is_hidden = {
                state_clone.window_hidden.lock()
                    .ok()
                    .map(|h| *h)
                    .unwrap_or(false)
            };

            if is_hidden {
                // Send notification if window is hidden to tray
                if let Err(e) = app_handle.notification()
                    .builder()
                    .title("学习提醒")
                    .body("该学习了！点击查看闪卡。")
                    .show() {
                    eprintln!("Failed to show notification: {}", e);
                }
            } else {
                // Show window if not hidden
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
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
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            let app_handle = app.handle().clone();

            // Load settings from persistent storage
            let store = app.store("settings.json")
                .expect("Failed to initialize store");

            let settings = Settings {
                interval: store.get("interval")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(600),  // Default 10 minutes in seconds
                autostart: store.get("autostart")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
                card_type: store.get("card_type")
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| "mixed".to_string()),
                close_behavior: Some(
                    store.get("close_behavior")
                        .and_then(|v| v.as_str().map(|s| s.to_string()))
                        .unwrap_or_else(|| "minimize".to_string())
                ),  // Default to minimize to tray
            };

            let state = AppState {
                settings: Arc::new(Mutex::new(settings)),
                timer_running: Arc::new(Mutex::new(true)),
                timer_generation: Arc::new(Mutex::new(0)),
                window_hidden: Arc::new(Mutex::new(false)),
            };

            app.manage(state.clone());

            // Create system tray
            let show_item = MenuItemBuilder::with_id("show", "显示窗口").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "完全退出").build(app)?;
            let menu = MenuBuilder::new(app)
                .item(&show_item)
                .item(&quit_item)
                .build()?;

            let tray_menu_state = state.clone();
            let tray_menu_show_state = state.clone();
            let tray_menu_show_app = app_handle.clone();
            let tray_quit_state = state.clone();
            let tray_click_state = state.clone();
            let tray_click_app = app_handle.clone();
            TrayIconBuilder::new()
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(move |app, event| {
                    match event.id().as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                                // Mark window as visible
                                if let Ok(mut hidden) = tray_menu_state.window_hidden.lock() {
                                    *hidden = false;
                                }
                                // Restart timer when window is shown
                                restart_timer(&tray_menu_show_app, &tray_menu_show_state);
                            }
                        }
                        "quit" => {
                            // Ask user to confirm exit
                            let state_clone = tray_quit_state.clone();
                            let app_clone = app.clone();
                            tauri::async_runtime::spawn(async move {
                                use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

                                let confirm = app_clone.dialog()
                                    .message("确定要完全退出应用吗？")
                                    .title("确认退出")
                                    .buttons(tauri_plugin_dialog::MessageDialogButtons::OkCancelCustom("确定".to_string(), "取消".to_string()))
                                    .kind(MessageDialogKind::Warning)
                                    .blocking_show();

                                if confirm {
                                    // Stop the timer task
                                    if let Ok(mut running) = state_clone.timer_running.lock() {
                                        *running = false;
                                    }
                                    std::process::exit(0);
                                }
                            });
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(move |_tray, event| {
                    // Only handle left-click to show window
                    // Right-click will show the menu automatically
                    if let TrayIconEvent::Click { button, .. } = event {
                        if button == tauri::tray::MouseButton::Left {
                            // Left-click: show window and restart timer
                            if let Some(window) = tray_click_app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                                // Mark window as visible and restart timer
                                if let Ok(mut hidden) = tray_click_state.window_hidden.lock() {
                                    *hidden = false;
                                }
                                restart_timer(&tray_click_app, &tray_click_state);
                            }
                        }
                        // Right-click shows menu automatically, no action needed
                    }
                })
                .build(app)?;

            // Restore window state for the main window created from tauri.conf.json
            if let Some(win) = app.get_webview_window("main") {
                let _ = win.restore_state(Default::default());

                // Setup handler for window close
                let close_state = state.clone();
                let close_app_handle = app_handle.clone();
                win.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        // Prevent default close behavior
                        api.prevent_close();

                        // Get close behavior from settings (always Some() with default "minimize")
                        let behavior = {
                            close_state.settings.lock()
                                .ok()
                                .and_then(|settings| settings.close_behavior.clone())
                                .unwrap_or_else(|| "minimize".to_string())
                        };

                        let state_clone = close_state.clone();
                        let app_clone = close_app_handle.clone();

                        // Execute close action directly based on settings
                        tauri::async_runtime::spawn(async move {
                            handle_close_action(&behavior, &state_clone, &app_clone).await;
                        });
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
