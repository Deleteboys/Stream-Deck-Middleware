// Modul-Deklarationen einbinden
mod protocol;
mod serial;

use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, RunEvent, State, WindowEvent,
};

use crate::protocol::HostToPico;

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

fn shutdown_app(app: &AppHandle) {
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
        let _ = tauri::WebviewWindowBuilder::new(
            app,
            "main",
            tauri::WebviewUrl::App("index.html".into()),
        )
        .title("Mein Programm")
        .build();
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
                api.prevent_close();
                let _ = window.destroy();
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app, event| {
        if let RunEvent::ExitRequested { api, .. } = event {
            let state = app.state::<AppState>();
            let should_exit = state.is_quitting.lock().map(|v| *v).unwrap_or(false);
            if !should_exit {
                api.prevent_exit();
            }
        }
    });
}
