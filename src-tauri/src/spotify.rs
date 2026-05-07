use rspotify::{prelude::*, scopes, AuthCodePkceSpotify, Config, Credentials, OAuth};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex as AsyncMutex;
use crate::config;

pub async fn authenticate(
    app: AppHandle,
    client_id: String,
    state_client: Arc<AsyncMutex<Option<AuthCodePkceSpotify>>>,
) -> Result<(), String> {

    let mut app_config = config::load_config(&app);
    app_config.spotify_client_id = Some(client_id.clone());
    let _ = config::save_config(&app, &app_config);

    let mut cache_path = app.path().app_data_dir().map_err(|e| e.to_string())?;

    std::fs::create_dir_all(&cache_path).map_err(|e| e.to_string())?;

    cache_path.push(".spotify_token");
    // ------------------------------------

    let creds = Credentials::new_pkce(&client_id);
    let scopes = scopes!(
        "user-modify-playback-state",
        "user-read-playback-state",
        "user-library-modify"
    );

    let oauth = OAuth {
        redirect_uri: "http://127.0.0.1:8888/callback".to_string(),
        scopes,
        ..Default::default()
    };

    // Wir übergeben unseren generierten Cache-Pfad an die Config
    let config = Config {
        token_cached: true,
        cache_path, // <--- NEU: Pfad hier einsetzen!
        ..Default::default()
    };

    let mut spotify = AuthCodePkceSpotify::with_config(creds, oauth, config);

    // 1. Zuerst schauen wir, ob wir schon einen Token im Cache haben (.spotify_token)
    if let Ok(Some(token)) = spotify.read_token_cache(false).await {
        *spotify.get_token().lock().await.unwrap() = Some(token);
        *state_client.lock().await = Some(spotify);
        println!("Spotify Login erfolgreich aus dem Cache geladen.");
        return Ok(());
    }

    println!("Kein Cache gefunden. Öffne Browser für Login...");

    // 2. Auth URL generieren und Browser öffnen (benötigt das 'open' crate)
    let url = spotify.get_authorize_url(None).map_err(|e| e.to_string())?;
    open::that(&url).map_err(|e| format!("Konnte Browser nicht öffnen: {}", e))?;

    // 3. Lokalen Async-Webserver starten, um den Callback abzufangen
    let listener = TcpListener::bind("127.0.0.1:8888")
        .await
        .map_err(|e| e.to_string())?;

    loop {
        let (mut stream, _) = listener.accept().await.map_err(|e| e.to_string())?;
        let mut buffer = [0; 2048];
        stream.read(&mut buffer).await.map_err(|e| e.to_string())?;

        let request = String::from_utf8_lossy(&buffer);

        // Wir warten auf den GET Request von Spotify
        if request.starts_with("GET /callback") {
            // Pfad extrahieren (z.B. "/callback?code=AQA...")
            let path = request.split_whitespace().nth(1).unwrap_or("");
            let full_url = format!("http://127.0.0.1:8888{}", path);

            // rspotify den Code aus der URL extrahieren lassen
            if let Some(code) = spotify.parse_response_code(&full_url) {
                // Den echten Token bei Spotify anfragen
                spotify.request_token(&code).await.map_err(|e| e.to_string())?;

                // Dem Browser eine hübsche Erfolgsmeldung schicken
                // Dem Browser eine hübsche Erfolgsmeldung schicken (inklusive UTF-8 Header!)
                let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\r\n<html><body style=\"background-color: #09090b; color: white; font-family: sans-serif; display: flex; flex-direction: column; justify-content: center; align-items: center; height: 100vh; margin: 0;\"><h1>Login erfolgreich! 🎵</h1><p>Du kannst dieses Fenster jetzt schließen.</p></body></html>";
                stream.write_all(response.as_bytes()).await.unwrap();

                // Den authentifizierten Client in den AppState speichern
                *state_client.lock().await = Some(spotify);
                println!("Spotify Login via Browser erfolgreich abgeschlossen.");
                break;
            }
        }
    }

    Ok(())
}


pub async fn init_from_cache(
    app: AppHandle,
    state_client: Arc<AsyncMutex<Option<AuthCodePkceSpotify>>>,
) {
    let app_config = config::load_config(&app);

    if let Some(client_id) = app_config.spotify_client_id {
        if let Ok(mut cache_path) = app.path().app_data_dir() {
            cache_path.push(".spotify_token");

            if cache_path.exists() {
                let creds = Credentials::new_pkce(&client_id);
                let oauth = OAuth {
                    redirect_uri: "http://127.0.0.1:8888/callback".to_string(),
                    ..Default::default()
                };

                let spotify = AuthCodePkceSpotify::with_config(
                    creds,
                    oauth,
                    Config {
                        token_cached: true,
                        cache_path,
                        ..Default::default()
                    },
                );

                if let Ok(Some(token)) = spotify.read_token_cache(false).await {
                    *spotify.get_token().lock().await.unwrap() = Some(token);
                    *state_client.lock().await = Some(spotify);
                    println!("Spotify erfolgreich im Hintergrund initialisiert (ID: {}).", client_id);
                }
            }
        }
    }
}