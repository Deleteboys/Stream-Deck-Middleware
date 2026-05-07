use crate::action::actions::Action;
use rspotify::model::{LibraryId, PlayableItem};
use rspotify::{prelude::*, AuthCodePkceSpotify, ClientResult};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use log::{debug, error, info, warn};
use tokio::sync::Mutex as AsyncMutex;

#[derive(Clone)]
pub struct SpotifyLikeAction {
    pub spotify: Arc<AsyncMutex<Option<AuthCodePkceSpotify>>>,
}

impl Debug for SpotifyLikeAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpotifyVolumeAction")
            .field("spotify", &"<SpotifyClient>")
            .finish()
    }
}

impl Action for SpotifyLikeAction {
    fn execute(&self) {
        let spotify_ptr = Arc::clone(&self.spotify);
        info!("SpotifyLikeAction: Starte 'Like'-Vorgang...");

        tauri::async_runtime::spawn(async move {
            let guard = spotify_ptr.lock().await;

            // 1. Überprüfen, ob der Spotify-Client überhaupt bereit ist
            let spotify = match guard.as_ref() {
                Some(s) => s,
                None => {
                    error!("Spotify-Client ist nicht initialisiert (None).");
                    return;
                }
            };

            debug!("Spotify-Client gesperrt, frage aktuellen Song ab...");

            // 2. Aktuellen Song abrufen mit Fehlerbehandlung für den Netzwerk-Call
            match spotify.current_user_playing_item().await {
                Ok(Some(song)) => {
                    // 3. Prüfen, ob es sich um einen Track handelt (und nicht um einen Podcast etc.)
                    if let Some(PlayableItem::Track(track)) = song.item {
                        if let Some(track_id) = track.id {
                            let track_id_string = track_id.to_string();
                            let track_name = track.name.clone();
                            let library_id = LibraryId::Track(track_id);

                            debug!("Versuche, Track '{}' ({:?}) zur Bibliothek hinzuzufügen...", track_name, track_id_string);

                            // 4. Song zur Library hinzufügen
                            match spotify.library_add(vec![library_id]).await {
                                Ok(_) => {
                                    info!("Erfolg: '{}' wurde zu deinen Lieblingssongs hinzugefügt.", track_name);
                                }
                                Err(e) => {
                                    error!("Fehler beim Hinzufügen von '{}': {:?}", track_name, e);
                                }
                            }
                        } else {
                            warn!("Der aktuelle Track hat keine gültige ID.");
                        }
                    } else {
                        warn!("Das aktuelle Element ist kein Track (evtl. eine Episode oder ein Podcast).");
                    }
                }
                Ok(None) => {
                    info!("Keine Aktion ausgeführt: Es wird momentan kein Song abgespielt.");
                }
                Err(e) => {
                    error!("Netzwerkfehler beim Abrufen des aktuellen Songs: {:?}", e);
                }
            }
        });
    }
}
