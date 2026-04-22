use std::time::Duration;
use std::io::{Read, Write};
use std::thread;
use std::sync::mpsc;
use serialport::{available_ports, SerialPortType};
use tauri::{AppHandle, Emitter}; // Wichtig für Tauri v2 Events
use crate::protocol::{HostToPico, PicoToHost};

pub fn start_serial_thread(app: AppHandle, rx: mpsc::Receiver<HostToPico>) {
    let port_name = match find_pico_port() {
        Some(p) => p,
        None => {
            println!("Pico nicht gefunden! Warte auf Gerät...");
            // Hier könnte man später eine Retry-Logik einbauen
            return;
        }
    };

    let mut port = serialport::new(&port_name, 115200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Port konnte nicht geöffnet werden");

    port.write_data_terminal_ready(true).ok();
    port.write_request_to_send(true).ok();
    thread::sleep(Duration::from_millis(500));
    port.clear(serialport::ClearBuffer::All).ok();

    println!("Serieller Hintergrund-Dienst läuft an {}...", port_name);

    let mut accumulator: Vec<u8> = Vec::new();
    let mut serial_buf = [0u8; 1024];

    loop {
        // --- A: Befehle von Tauri empfangen und an Pico senden ---
        if let Ok(cmd) = rx.try_recv() {
            let mut buf = [0u8; 64];
            if let Ok(slice) = postcard::to_slice(&cmd, &mut buf) {
                let _ = port.write_all(slice);
                let _ = port.flush();
            }
        }

        // --- B: Daten vom Pico lesen und an Tauri-Frontend feuern ---
        if let Ok(bytes_read) = port.read(&mut serial_buf) {
            if bytes_read > 0 {
                accumulator.extend_from_slice(&serial_buf[..bytes_read]);
                loop {
                    match postcard::take_from_bytes::<PicoToHost>(&accumulator) {
                        Ok((msg, rest)) => {
                            // Sende das Event an dein Frontend (JS/TS)
                            // In Tauri v2 nutzt man `emit` statt `emit_all`
                            if let Err(e) = app.emit("pico-event", msg) {
                                println!("Fehler beim Senden ans Frontend: {}", e);
                            }
                            accumulator = rest.to_vec();
                        }
                        Err(postcard::Error::DeserializeUnexpectedEnd) => break,
                        Err(_) => {
                            if !accumulator.is_empty() { accumulator.remove(0); } else { break; }
                        }
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(1));
    }
}

fn find_pico_port() -> Option<String> {
    available_ports()
        .unwrap_or_default()
        .into_iter()
        .find(|p| {
            if let SerialPortType::UsbPort(info) = &p.port_type {
                return info.vid == 0xc0de && info.pid == 0xcafe;
            }
            false
        })
        .map(|p| p.port_name)
}