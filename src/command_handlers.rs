use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::error::Error;
use slog::error;

use colored::Colorize;
use walkdir::WalkDir;

use crate::db;
use crate::shazam;
use crate::download;
use crate::utils;
use crate::wav;
use crate::models;

// Placeholder import for a socket.io–like server in Rust.
// TODO: Server implementation
// use socketio_server::{Server, Conn, EngineOptions, Transport};

const SONGS_DIR: &str = "songs";

pub async fn find(file_path: &str) {
    // Convert relative path to absolute for better error reporting
    let absolute_path = std::path::Path::new(file_path)
        .canonicalize()
        .unwrap_or_else(|_| std::path::PathBuf::from(file_path));
    
    println!("Attempting to read file: {}", absolute_path.display());
    
    if !std::path::Path::new(file_path).exists() {
        println!("{}", format!("Error: File '{}' does not exist", file_path).yellow());
        return;
    }

    // Check if file is MP3 and convert to WAV first
    let path_obj = Path::new(file_path);
    let file_ext = path_obj.extension()
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    
    let wav_file_path = if file_ext.to_lowercase() == "mp3" {
        match wav::convert_to_wav(file_path, 1) {
            Ok(wav_path) => wav_path,
            Err(e) => {
                println!("{}", format!("Error converting MP3 to WAV: {:?}", e).yellow());
                return;
            }
        }
    } else {
        file_path.to_string()
    };

    let wav_info = match wav::read_wav_info(&wav_file_path) {
        Ok(info) => info,
        Err(e) => {
            println!("{}", format!("Error reading wave info: {:?}", e).yellow());
            return;
        }
    };

    let samples = match wav::wav_bytes_to_samples(&wav_info.data) {
        Ok(s) => s,
        Err(e) => {
            println!("{}", format!("Error converting to samples: {:?}", e).yellow());
            return;
        }
    };

    let (matches, search_duration) =
        match shazam::find_matches(&samples, wav_info.duration, wav_info.sample_rate).await {
            Ok(result) => result,
            Err(e) => {
                println!("{}", format!("Error finding matches: {:?}", e).yellow());
                return;
            }
        };

    if matches.is_empty() {
        println!("\nNo match found.");
        println!("\nSearch took: {:?}", search_duration);
        return;
    }

    let (msg, top_matches) = if matches.len() >= 20 {
        ("Top 20 matches:", &matches[..20])
    } else {
        ("Matches:", &matches[..])
    };

    println!("{}", msg);
    for m in top_matches {
        println!(
            "\t- {} by {}, score: {:.2}",
            m.song_title, m.song_artist, m.score
        );
    }
    println!("\nSearch took: {:?}", search_duration);

    let top_match = &top_matches[0];
    println!(
        "\nFinal prediction: {} by {} , score: {:.2}",
        top_match.song_title, top_match.song_artist, top_match.score
    );
}

pub fn download(spotify_url: &str) {
    if let Err(e) = utils::create_folder(SONGS_DIR) {
        let wrapped_err = utils::wrap_error(e);
        let logger = utils::get_logger();
        let msg = format!("failed to create directory {}", SONGS_DIR);
        error!(logger, "{}", msg; "error" => wrapped_err.to_string());

    }

    if spotify_url.contains("album") {
        if let Err(e) = download::dl_album(spotify_url, SONGS_DIR) {
            println!("{}", format!("Error: {:?}", e).yellow());
        }
    }

    if spotify_url.contains("playlist") {
        if let Err(e) = download::dl_playlist(spotify_url, SONGS_DIR) {
            println!("{}", format!("Error: {:?}", e).yellow());
        }
    }

    if spotify_url.contains("track") {
        if let Err(e) = download::dl_single_track(spotify_url, SONGS_DIR) {
            println!("{}", format!("Error: {:?}", e).yellow());
        }
    }
}

// pub fn serve(protocol: &str, port: &str) {
//     let protocol = protocol.to_lowercase();
//     let allow_origin = |_: &http::Request<()>| true;

//     let engine_options = EngineOptions {
//         transports: vec![
//             Transport::Polling { check_origin: Box::new(allow_origin.clone()) },
//             Transport::Websocket { check_origin: Box::new(allow_origin) },
//         ],
//         ..Default::default()
//     };

//     let mut server = Server::new(engine_options);

//     server.on_connect("/", |socket: Conn| {
//         socket.set_context("");
//         println!("CONNECTED: {}", socket.id());
//         Ok(())
//     });

//     server.on_event("/", "totalSongs", utils::handle_total_songs);
//     server.on_event("/", "newDownload", utils::handle_song_download);
//     server.on_event("/", "newRecording", utils::handle_new_recording);

//     server.on_error("/", |socket: Conn, e: Box<dyn Error>| {
//         println!("meet error: {:?}", e);
//     });

//     server.on_disconnect("/", |socket: Conn, reason: &str| {
//         println!("closed {}", reason);
//     });

//     // Run the socket.io server in a separate thread.
//     std::thread::spawn(move || {
//         if let Err(e) = server.serve() {
//             panic!("socketio listen error: {}", e);
//         }
//     });

//     let serve_https = protocol == "https";
//     serve_http(&mut server, serve_https, port);
// }

// pub fn serve_http(
//     socket_server: &mut dyn socketio_server::SocketIOServer,
//     serve_https: bool,
//     port: &str,
// ) {
//     use hyper::service::{make_service_fn, service_fn};
//     use hyper::{Body, Request, Response, Server as HttpServer};
//     use std::net::SocketAddr;
//     use futures::executor;

//     // Create a hyper service that delegates to the socket server.
//     let make_svc = make_service_fn(|_conn| async {
//         Ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| async move {
//             socket_server.handle(req).await
//         }))
//     });

//     let addr = SocketAddr::from(([0, 0, 0, 0], port.parse().unwrap()));

//     if serve_https {
//         let cert_key_default = "/etc/letsencrypt/live/localport.online/privkey.pem";
//         let cert_file_default = "/etc/letsencrypt/live/localport.online/fullchain.pem";

//         let cert_key = utils::get_env("CERT_KEY", cert_key_default);
//         let cert_file = utils::get_env("CERT_FILE", cert_file_default);
//         if cert_key.is_empty() || cert_file.is_empty() {
//             panic!("Missing cert");
//         }

//         println!("Starting HTTPS server on {:?}", addr);
//         // For HTTPS, assume utils::start_https_server is implemented (e.g. using hyper-rustls).
//         if let Err(e) = utils::start_https_server(addr, &cert_file, &cert_key, make_svc) {
//             panic!("HTTPS server error: {}", e);
//         }
//     } else {
//         println!("Starting HTTP server on port {}", port);
//         let server = HttpServer::bind(&addr).serve(make_svc);
//         if let Err(e) = executor::block_on(server) {
//             panic!("HTTP server error: {}", e);
//         }
//     }
// }

pub async fn erase(songs_dir: &str) {
    let logger = utils::get_logger();

    // Wipe database collections.
    let mut db_client = match db::new_db_client().await {
        Ok(client) => client,
        Err(e) => {
            let msg = format!("Error creating DB client: {:?}", e);

            // logger.error(&msg, &e);
            error!(logger, "{}", msg; "error" => e.to_string());
            return;
        }
    };

    if let Err(e) = db_client.delete_collection("fingerprints") {
        let msg = format!("Error deleting collection: {:?}", e);

        error!(logger, "{}", msg; "error" => e.to_string());

    }

    if let Err(e) = db_client.delete_collection("songs") {
        let msg = format!("Error deleting collection: {:?}", e);

        error!(logger, "{}", msg; "error" => e.to_string());

    }

    // Delete song files.
    if let Err(e) = WalkDir::new(songs_dir).into_iter().try_for_each(|entry| {
        let entry = entry?;
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension().and_then(|s| s.to_str()) {
                if ext == "wav" || ext == "m4a" {
                    fs::remove_file(entry.path())?;
                }
            }
        }
        Ok::<(), io::Error>(())
    }) {
        let msg = format!("Error walking through directory {}: {:?}", songs_dir, e);

        error!(logger, "{}", msg; "error" => e.to_string());

    }

    println!("Erase complete");
}

pub fn save(path: &str, force: bool) {
    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(e) => {
            println!("Error stating path {}: {:?}", path, e);
            return;
        }
    };

    if metadata.is_dir() {
        for entry in WalkDir::new(path) {
            match entry {
                Ok(entry) if entry.file_type().is_file() => {
                    if let Err(e) = save_song(entry.path(), force) {
                        println!("Error saving song ({}): {:?}", entry.path().display(), e);
                    }
                }
                Err(e) => {
                    println!("Error walking the path {}: {:?}", path, e);
                }
                _ => {}
            }
        }
    } else {
        if let Err(e) = save_song(Path::new(path), force) {
            println!("Error saving song ({}): {:?}", path, e);
        }
    }
}

pub fn save_song(file_path: &Path, force: bool) -> Result<(), Box<dyn Error>> {

    let file_ext = file_path.extension()
    .and_then(|s| s.to_str())
    .unwrap_or_default();

if file_ext.to_lowercase() == "mp3" {
    // First convert MP3 to WAV before proceeding
    let wav_path = match wav::convert_to_wav(file_path.to_str().unwrap_or_default(), 1) {
        Ok(path) => path,
        Err(e) => {
            return Err(format!("Failed to convert MP3 to WAV: {:?}", e).into());
        }
    };
    // Continue with the converted file
    return save_song(&Path::new(&wav_path), force);
}

    let metadata = wav::get_metadata(file_path.to_str().ok_or("Invalid path")?)?;
    let duration_float: f64 = metadata.format.duration.parse().map_err(|e| {
        format!("failed to parse duration to float: {:?}", e)
    })?;

    let tags = metadata.format.tags.unwrap_or_default();
    let track = models::Track {
        album: tags.get("album").cloned().unwrap_or_default(),
        artist: tags.get("artist").cloned().unwrap_or_default(),
        artists: Vec::new(),
        title: tags.get("title").cloned().unwrap_or_default(),
        duration: duration_float.round() as f64,
    };

    let yt_id = match download::get_youtube_id(&track) {
        Ok(id) => id,
        Err(e) if !force => {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!("failed to get YouTube ID for song: {:?}", e),
            )))
        }
        Err(_) => String::new(),
    };

    if track.title.is_empty() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "no title found in metadata",
        )));
    }
    if track.artist.is_empty() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "no artist found in metadata",
        )));
    }

    download::process_and_save_song(file_path.to_str().ok_or("Invalid path")?, &track.title, &track.artist, &yt_id)
        .map_err(|e| format!("failed to process or save song: {:?}", e))?;

    let file_stem = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    let wav_file = format!("{}.wav", file_stem);
    let source_path = file_path.with_file_name(&wav_file);
    let new_file_path = Path::new(SONGS_DIR).join(&wav_file);
    fs::rename(source_path, new_file_path)
        .map_err(|e| format!("failed to rename temporary file to output file: {:?}", e))?;
    Ok(())
}
