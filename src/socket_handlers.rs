use serde_json::json;
use slog::error;
use slog::info;

use crate::db;
use crate::models;
use crate::shazam;
use crate::download;
use crate::utils;
//
// Assume a SocketIOSocket trait is defined somewhere in your project that resembles:
// 
// pub trait SocketIOSocket {
//     fn emit(&self, event: &str, message: &str);
// }
// 
// For example:
// 
// pub struct Socket;
// impl SocketIOSocket for Socket {
//     fn emit(&self, event: &str, message: &str) {
//         // implementation here
//     }
// }
//
pub trait SocketIOSocket {
    fn emit(&self, event: &str, message: &str);
}

/// Helper function to create a JSON string representing a download status.
pub fn download_status(status_type: &str, message: &str) -> String {
    let data = json!({
        "type": status_type,
        "message": message,
    });
    match serde_json::to_string(&data) {
        Ok(json_data) => json_data,
        Err(e) => {
            let logger = utils::get_logger();
            let err = utils::wrap_error(e);
            error!(logger, "failed to download marshal data: {}", err);

            String::new()
        }
    }
}

/// Handler for total songs socket event.
pub async fn handle_total_songs(socket: &dyn SocketIOSocket) {
    let logger = utils::get_logger();

    let db_client = match db::new_db_client().await {
        Ok(client) => client,
        Err(e) => {
            error!(logger, "error connecting to DB: {}", e);
            return;
        }
    };

    let total_songs = match db_client.total_songs() {
        Ok(total) => total,
        Err(e) => {
            error!(logger, "Log error getting total songs: {}", e);
            
            return;
        }
    };

    socket.emit("totalSongs", &total_songs.to_string());
}

/// Handler for song download events from a socket.
pub async fn handle_song_download(socket: &dyn SocketIOSocket, spotify_url: &str) {
    let logger = utils::get_logger();

    if spotify_url.contains("album") {
        match download::album_info(spotify_url) {
            Ok(tracks_in_album) => {
                let status_msg = format!("{} songs found in album.", tracks_in_album.len());
                socket.emit("downloadStatus", &download_status("info", &status_msg));

                match download::dl_album(spotify_url, utils::SONGS_DIR) {
                    Ok(total_tracks_downloaded) => {
                        let status_msg = format!("{} songs downloaded from album", total_tracks_downloaded);
                        socket.emit("downloadStatus", &download_status("success", &status_msg));
                    }
                    Err(e) => {
                        socket.emit("downloadStatus", &download_status("error", "Couldn't download album."));
                        // logger.error_context("failed to download album.", e);
                        // error_context(&logger, "", err);
                        error!(logger, "failed to download album. {}", e);

                    }
                }
            }
            Err(e) => {
                if e.to_string().len() <= 25 {
                    socket.emit("downloadStatus", &download_status("error", &e.to_string()));
                    // logger.info(&e.to_string());
                    info!(logger, "{}", e.to_string());
                } else {
                    // logger.error_context("", e);
                    error!(logger, "error getting album info {}", e);
                }
                return;
            }
        }
    }

    if spotify_url.contains("playlist") {
        match download::playlist_info(spotify_url) {
            Ok(tracks_in_pl) => {
                let status_msg = format!("{} songs found in playlist.", tracks_in_pl.len());
                socket.emit("downloadStatus", &download_status("info", &status_msg));

                match download::dl_playlist(spotify_url, utils::SONGS_DIR) {
                    Ok(total_tracks_downloaded) => {
                        let status_msg = format!("{} songs downloaded from playlist.", total_tracks_downloaded);
                        socket.emit("downloadStatus", &download_status("success", &status_msg));
                    }
                    Err(e) => {
                        socket.emit("downloadStatus", &download_status("error", "Couldn't download playlist."));
                        // logger.error_context("", e);
                        error!(logger, "failed to download playlist. {}", e);
                    }
                }
            }
            Err(e) => {
                if e.to_string().len() <= 25 {
                    socket.emit("downloadStatus", &download_status("error", &e.to_string()));
                    // logger.info(&e.to_string());
                    info!(logger, "{}", e.to_string());
                } else {
                    // logger.error_context("error getting playlist info", e);
                    error!(logger, "error getting playlist info {}", e);
                }
                return;
            }
        }
    }

    if spotify_url.contains("track") {
        let track_info = match download::track_info(spotify_url) {
            Ok(info) => info,
            Err(e) => {
                if e.to_string().len() <= 25 {
                    socket.emit("downloadStatus", &download_status("error", &e.to_string()));
                    // logger.info(&e.to_string());
                    info!(logger, "{}", e.to_string());
                } else {
                    // logger.error_context("error getting track info", e);
                    error!(logger, "error getting track info {}", e);
                }
                return;
            }
        };

        // Check if track already exists in DB.
        let db_client = match db::new_db_client().await {
                    Ok(client) => client,
                    Err(e) => {
                        error!(logger, "error connecting to DB {}", e);
                        return;
                    }
                };

        match db_client.get_song_by_key(&utils::generate_song_key(&track_info.title, &track_info.artist)) {
            Ok((song, song_exists)) => {
                if song_exists {
                    let status_msg = format!(
                        "'{}' by '{}' already exists in the database (https://www.youtube.com/watch?v={})",
                        song.title, song.artist, song.youtube_id
                    );
                    socket.emit("downloadStatus", &download_status("error", &status_msg));
                    return;
                }
            }
            Err(e) => {
                // logger.error_context("", e);
                error!(logger, "failed to get song by key.{}", e);
            }
        }

        match download::dl_single_track(spotify_url, utils::SONGS_DIR) {
            Ok(total_downloads) => {
                if total_downloads != 1 {
                    let status_msg = format!("'{}' by '{}' failed to download", track_info.title, track_info.artist);
                    socket.emit("downloadStatus", &download_status("error", &status_msg));
                } else {
                    let status_msg = format!("'{}' by '{}' was downloaded", track_info.title, track_info.artist);
                    socket.emit("downloadStatus", &download_status("success", &status_msg));
                }
            }
            Err(e) => {
                if e.to_string().len() <= 25 {
                    socket.emit("downloadStatus", &download_status("error", &e.to_string()));
                    // logger.info(&e.to_string());
                    info!(logger, "{}", e.to_string());
                } else {
                    // logger.error_context("error ", e);
                    error!(logger, "error downloading track{}", e);
                }
                return;
            }
        }
    }
}

/// Handler for new recording events from a socket.
pub async fn handle_new_recording(socket: &dyn SocketIOSocket, record_data: &str) {
    let logger = utils::get_logger();

    let rec_data: models::RecordData = match serde_json::from_str(record_data) {
        Ok(data) => data,
        Err(e) => {
            error!(logger, "Failed to unmarshal record data. {}", e);
            return;
        }
    };

    let samples = match utils::process_recording(&rec_data, true) {
        Ok(s) => s,
        Err(e) => {
            error!(logger, "Failed to process recording. {}", e);
            return;
        }
    };

    let (matches, _duration) =
        match shazam::find_matches(&samples, rec_data.duration, rec_data.sample_rate).await {
            Ok(result) => result,
            Err(e) => {
                error!(logger, "failed to get matches.{}", e);
                return;
            }
        };

    // Only return up to 10 matches.
    let json_data: String = match serde_json::to_string(if matches.len() > 10 {
        &matches[..10]
    } else {
        &matches[..]
    }) {
        Ok(data) => data,
        Err(e) => {
            error!(logger, "failed to marshal matches. {}", e);
            return;
        }
    };

    socket.emit("matches", &json_data);
}
