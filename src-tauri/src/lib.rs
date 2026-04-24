// Modul-Deklarationen einbinden
mod protocol;
mod serial;
pub mod action; // Das bindet den Ordner "action" über die mod.rs ein

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, RunEvent, State, WindowEvent,
};

use crate::protocol::HostToPico;

const WINDOW_STATE_FILE: &str = "window-state.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistedWindowState {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    maximized: bool,
}

// State, um vom UI Befehle an den Hintergrund-Thread zu schicken
struct AppState {
    serial_tx: Mutex<Option<mpsc::Sender<HostToPico>>>,
    is_quitting: Mutex<bool>,
}

// Der Command, den dein JavaScript aufruft
#[tauri::command]
fn send_to_pico(state: State<AppState>, command: HostToPico) -> Result<(), String> {
    let tx_guard = state.serial_tx.lock().unwrap();
    if let Some(tx) = tx_guard.as_ref() {
        tx.send(command).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Keine Verbindung zum seriellen Thread".into())
    }
}

fn is_quitting(app: &AppHandle) -> bool {
    app.state::<AppState>()
        .is_quitting
        .lock()
        .map(|v| *v)
        .unwrap_or(false)
}

fn window_state_path(app: &AppHandle) -> Option<PathBuf> {
    let mut base = app.path().app_data_dir().ok()?;
    if fs::create_dir_all(&base).is_err() {
        return None;
    }
    base.push(WINDOW_STATE_FILE);
    Some(base)
}

fn capture_window_state(window: &tauri::WebviewWindow) -> Option<PersistedWindowState> {
    let position = window.outer_position().ok()?;
    let size = window.outer_size().ok()?;

    Some(PersistedWindowState {
        x: position.x,
        y: position.y,
        width: size.width,
        height: size.height,
        maximized: window.is_maximized().unwrap_or(false),
    })
}

fn persist_window_state(window: &tauri::WebviewWindow) {
    let Some(state) = capture_window_state(window) else {
        return;
    };
    let Some(path) = window_state_path(&window.app_handle()) else {
        return;
    };

    if let Ok(serialized) = serde_json::to_string(&state) {
        let _ = fs::write(path, serialized);
    }
}

fn load_window_state(app: &AppHandle) -> Option<PersistedWindowState> {
    let path = window_state_path(app)?;
    let content = fs::read_to_string(path).ok()?;
    let state: PersistedWindowState = serde_json::from_str(&content).ok()?;

    if state.width == 0 || state.height == 0 {
        return None;
    }

    Some(state)
}

fn apply_window_state(window: &tauri::WebviewWindow, state: &PersistedWindowState) {
    if state.maximized {
        let _ = window.maximize();
        return;
    }

    let _ = window.unmaximize();
    let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize::new(
        state.width,
        state.height,
    )));
    let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(
        state.x, state.y,
    )));
}

fn shutdown_app(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        persist_window_state(&window);
    }

    let state = app.state::<AppState>();
    if let Ok(mut quitting) = state.is_quitting.lock() {
        *quitting = true;
    }

    // Sender droppen, damit der Serial-Thread uber "Disconnected" sauber aussteigt.
    if let Ok(mut tx_guard) = state.serial_tx.lock() {
        tx_guard.take();
    }

    app.exit(0);
}

fn show_or_create_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    } else {
        let window = tauri::WebviewWindowBuilder::new(
            app,
            "main",
            tauri::WebviewUrl::App("index.html".into()),
        )
        .title("Mein Programm")
        .build();

        if let Ok(window) = window {
            if let Some(state) = load_window_state(app) {
                apply_window_state(&window, &state);
            }
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Channel fur den seriellen Thread erstellen
    let (tx, rx) = mpsc::channel::<HostToPico>();

    let app = tauri::Builder::default()
        // 1. AppState registrieren, damit `send_to_pico` darauf zugreifen kann
        .manage(AppState {
            serial_tx: Mutex::new(Some(tx)),
            is_quitting: Mutex::new(false),
        })
        // 2. Den Command fur das Frontend registrieren
        .invoke_handler(tauri::generate_handler![send_to_pico])
        .setup(|app| {
            // --- TRAY MENU SETUP ---
            let quit_i = MenuItem::with_id(app, "quit", "Beenden", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Einstellungen", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        shutdown_app(app);
                    }
                    "show" => {
                        show_or_create_main_window(app);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        show_or_create_main_window(&app);
                    }
                })
                .build(app)?;

            // --- SERIELLER THREAD SETUP ---
            let app_handle = app.handle().clone();
            thread::spawn(move || {
                serial::start_serial_thread(app_handle, rx);
            });

            // --- FENSTER LOGIK ---
            if let Some(window) = app.get_webview_window("main") {
                if let Some(state) = load_window_state(&app.handle()) {
                    apply_window_state(&window, &state);
                }
            }

            let should_show_gui = true; // Placeholder fur Settings
            if !should_show_gui {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                if let Some(webview_window) = window.app_handle().get_webview_window(window.label()) {
                    persist_window_state(&webview_window);
                }

                if is_quitting(&window.app_handle()) {
                    return;
                }

                api.prevent_close();
                let _ = window.hide();
            }
            WindowEvent::Moved(_) | WindowEvent::Resized(_) => {
                if let Some(webview_window) = window.app_handle().get_webview_window(window.label()) {
                    persist_window_state(&webview_window);
                }
            }
            WindowEvent::Focused(false) => {
                if let Some(webview_window) = window.app_handle().get_webview_window(window.label()) {
                    persist_window_state(&webview_window);
                }
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app, event| {
        if let RunEvent::ExitRequested { api, .. } = event {
            if let Some(window) = app.get_webview_window("main") {
                persist_window_state(&window);
            }

            let state = app.state::<AppState>();
            let should_exit = state.is_quitting.lock().map(|v| *v).unwrap_or(false);
            if !should_exit {
                api.prevent_exit();
            }
        }
    });
}
