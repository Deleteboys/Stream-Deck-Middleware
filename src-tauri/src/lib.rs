// Modul-Deklarationen einbinden
mod protocol;
mod serial;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent, State
};
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;

use crate::protocol::HostToPico;

// State, um vom UI Befehle an den Hintergrund-Thread zu schicken
struct AppState {
    serial_tx: Mutex<Option<mpsc::Sender<HostToPico>>>,
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Channel für den seriellen Thread erstellen
    let (tx, rx) = mpsc::channel::<HostToPico>();

    tauri::Builder::default()
        // 1. AppState registrieren, damit `send_to_pico` darauf zugreifen kann
        .manage(AppState {
            serial_tx: Mutex::new(Some(tx)),
        })
        // 2. Den Command für das Frontend registrieren
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
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button: tauri::tray::MouseButton::Left, .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // --- SERIELLER THREAD SETUP ---
            let app_handle = app.handle().clone();
            thread::spawn(move || {
                serial::start_serial_thread(app_handle, rx);
            });

            // --- FENSTER LOGIK ---
            let should_show_gui = true; // Placeholder für Settings
            if !should_show_gui {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}