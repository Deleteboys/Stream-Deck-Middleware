use crate::action::actions::Action;
use std::fmt::Debug;
use sysinfo::{System, ProcessesToUpdate};
use windows::core::Interface;
use windows::Win32::Media::Audio::*;
use windows::Win32::System::Com::*;

#[derive(Debug, Clone)]
pub struct AppVolumeAction {
    pub process_name: String,
    pub step: i8, // z.B. 5 für +5%, -5 für -5%
}

/// Hilfsfunktion, um die Lautstärke für alle Instanzen eines Prozesses anzupassen
pub unsafe fn adjust_app_volume(target_name: &str, step: i8) -> windows::core::Result<()> {
    let mut sys = System::new_all();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    // Alle PIDs finden, die zum Prozessnamen gehören
    let target_pids: Vec<u32> = sys.processes().iter()
        .filter(|(_, p)| p.name().to_string_lossy() == target_name)
        .map(|(pid, _)| pid.as_u32())
        .collect();

    if target_pids.is_empty() {
        return Ok(());
    }

    // COM-Schnittstellen initialisieren
    let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
    let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
    let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;
    let manager: IAudioSessionManager2 = device.Activate::<IAudioSessionManager2>(CLSCTX_ALL, None)?;

    let session_enumerator = manager.GetSessionEnumerator()?;
    let session_count = session_enumerator.GetCount()?;

    let step_float = step as f32 / 100.0;

    for i in 0..session_count {
        if let Ok(session) = session_enumerator.GetSession(i) {
            if let Ok(session2) = session.cast::<IAudioSessionControl2>() {
                if let Ok(pid) = session2.GetProcessId() {

                    // Prüfen, ob die Session zu einem unserer Ziel-PIDs gehört
                    if target_pids.contains(&pid) {
                        if let Ok(simple_volume) = session.cast::<ISimpleAudioVolume>() {
                            // Aktuelle Lautstärke der App abfragen
                            let current_vol = simple_volume.GetMasterVolume()?;

                            // Neue Lautstärke berechnen und limitieren (0.0 bis 1.0)
                            let new_vol = (current_vol + step_float).clamp(0.0, 1.0);

                            simple_volume.SetMasterVolume(new_vol, std::ptr::null())?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

impl Action for AppVolumeAction {
    fn execute(&self) {
        let name = self.process_name.clone();
        let step = self.step;

        tauri::async_runtime::spawn(async move {
            unsafe {
                if let Err(e) = adjust_app_volume(&name, step) {
                    println!("Fehler beim Ändern der Lautstärke für {}: {}", name, e);
                } else {
                    println!("Lautstärke für {} um {}% angepasst.", name, step);
                }
            }
        });
    }
}