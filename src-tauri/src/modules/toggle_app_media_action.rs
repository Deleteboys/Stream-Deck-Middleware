use crate::action::actions::Action;
use std::fmt::Debug;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

#[derive(Debug, Clone)]
pub struct ToggleAppMediaAction {
    pub process_name: String,
}

impl Action for ToggleAppMediaAction {
    fn execute(&self) {
        let name = self.process_name.clone();

        tauri::async_runtime::spawn(async move {
            let result: windows::core::Result<()> = async {
                let target_app = name.to_lowercase();

                let mut pending_operations = Vec::new();
                {
                    let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.await?;
                    let sessions = manager.GetSessions()?;

                    for session in sessions {
                        let app_id = session.SourceAppUserModelId()?.to_string();

                        if app_id.to_lowercase().contains(&target_app) {
                            let op = session.TryTogglePlayPauseAsync()?;
                            pending_operations.push((app_id, op));
                        }
                    }
                }
                for (app_id, op) in pending_operations {
                    op.await?;
                    println!("Media-Status (Play/Pause) für {} getoggelt.", app_id);
                }

                Ok(())
            }.await;

            if let Err(e) = result {
                println!("Fehler beim Toggeln der Medien für {}: {}", name, e);
            }
        });
    }
}