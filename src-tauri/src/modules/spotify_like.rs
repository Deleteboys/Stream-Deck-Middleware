use crate::action::actions::Action;
use rspotify::model::{LibraryId, PlayableItem};
use rspotify::{prelude::*, AuthCodePkceSpotify, ClientResult};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use log::{error, info};
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

        tauri::async_runtime::spawn(async move {
            let guard = spotify_ptr.lock().await;

            if let Some(spotify) = guard.as_ref() {
                let current_song = spotify.current_user_playing_item().await.unwrap();
                if let Some(song) = current_song {
                    if let Some(PlayableItem::Track(track)) = song.item {
                        if let Some(track_id) = track.id {
                            let library_id = LibraryId::Track(track_id);
                            match spotify.library_add(vec![library_id]).await {

                                Ok(_) => {info!("The song {} was successfully added to your library", track.name)}
                                Err(e) => {error!("Failed to add song to library: {:?}", e)}

                            }
                        }
                    }
                }
            }
        });
    }
}
