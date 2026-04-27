use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};
use crate::action::actions::Action;
use crate::audio::adjust_volume_for_pids;

#[derive(Debug, Clone)]
pub struct ForegroundVolumeAction {
    pub step: i8,
}

impl Action for ForegroundVolumeAction {
    fn execute(&self) {
        let step = self.step;

        tauri::async_runtime::spawn(async move {
            unsafe {
                let hwnd = GetForegroundWindow();
                if hwnd.is_invalid() {
                    return;
                }

                let mut pid: u32 = 0;
                GetWindowThreadProcessId(hwnd, Some(&mut pid));

                if pid != 0 {
                    if let Err(e) = adjust_volume_for_pids(&[pid], step) {
                        println!("Fehler beim Ändern des Vordergrund-Programms: {}", e);
                    } else {
                        println!("Vordergrund-Lautstärke angepasst (PID: {})", pid);
                    }
                }
            }
        });
    }
}