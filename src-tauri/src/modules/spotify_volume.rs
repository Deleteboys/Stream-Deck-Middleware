use std::fmt;
use std::fmt::{Debug, Formatter};
use crate::action::actions::Action;
use rspotify::{prelude::*, AuthCodePkceSpotify};
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;

#[derive(Clone)]
pub struct SpotifyVolumeAction {
    pub step: i8,
    pub spotify: Arc<AsyncMutex<Option<AuthCodePkceSpotify>>>,
}

impl Debug for SpotifyVolumeAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpotifyVolumeAction")
            .field("step", &self.step)
            .field("spotify", &"<SpotifyClient>")
            .finish()
    }
}

impl Action for SpotifyVolumeAction {
    fn execute(&self) {
        let step = self.step;
        let spotify_ptr = Arc::clone(&self.spotify);

        // Da die API-Abfrage asynchron ist, nutzen wir die Tauri-Runtime
        tauri::async_runtime::spawn(async move {
            // Sperre den Mutex, um sicher auf den Client zuzugreifen
            let guard = spotify_ptr.lock().await;

            if let Some(spotify) = guard.as_ref() {
                // 1. Aktuelle Wiedergabe abrufen (enthält das aktive Gerät und die Lautstärke)
                match spotify.current_playback(None, None::<Vec<_>>).await {
                    Ok(Some(playback)) => {
                        if let Some(current_vol) = playback.device.volume_percent {
                            // 2. Neue Lautstärke berechnen (clamp stellt sicher, dass wir zwischen 0 und 100 bleiben)
                            let new_vol = (current_vol as i16 + step as i16).clamp(0, 100) as u8;

                            // 3. Neue Lautstärke über die API setzen
                            if let Err(e) = spotify.volume(new_vol, None).await {
                                eprintln!("Spotify API Fehler beim Setzen der Lautstärke: {}", e);
                            } else {
                                println!(
                                    "Spotify Volume angepasst: {}% -> {}%",
                                    current_vol, new_vol
                                );
                            }
                        } else {
                            println!("Das aktive Spotify-Gerät meldet keine Lautstärke-Unterstützung.");
                        }
                    }
                    Ok(None) => {
                        println!("Keine aktive Spotify-Wiedergabe gefunden. Läuft gerade Musik?");
                    }
                    Err(e) => {
                        eprintln!("Fehler beim Abrufen der Spotify-Wiedergabe: {}", e);
                    }
                }
            } else {
                println!("Spotify-Action abgebrochen: Kein aktiver Login gefunden. Bitte in den Einstellungen einloggen.");
            }
        });
    }
}