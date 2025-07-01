use actix_web::{web, App, HttpServer, HttpResponse, Responder, Error, middleware::DefaultHeaders};
use actix_multipart::Multipart;
use actix_cors::Cors;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

use crate::command_handlers;
use crate::utils;
use crate::shazam;
use crate::wav;
use crate::db;

// For Spotify URL requests
#[derive(Deserialize)]
struct SpotifyUrl {
    url: String,
}

// For save song options
#[derive(Deserialize)]
struct SaveOptions {
    force: Option<bool>,
}

// Add this struct for the library response
#[derive(Serialize)]
struct LibrarySong {
    id: u32,
    name: String,
    artist: String,
    youtube_id: String,
    thumbnail_url: Option<String>,
}

// API endpoint for finding songs
async fn api_find(mut payload: Multipart) -> Result<impl Responder, Error> {
    // Save the uploaded file to a temporary location
    let mut temp_file = NamedTempFile::new()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    let file_path = temp_file.path().to_string_lossy().to_string();
    
    // Process uploaded file
    let mut filename = String::new();
    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        
        // Attempt to get the field name and filename
        if let Some(content_disposition) = field.content_disposition() {
            if let Some(name) = content_disposition.get_name() {
                if name == "file" { // Assuming the field name is "file"
                    if let Some(fname) = content_disposition.get_filename() {
                        filename = fname.to_string();
                    }
                }
            }
        }
        
        // Read and write file data
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            temp_file.write_all(&data)
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        }
    }
    
    // Make sure to flush the file to ensure all data is written
    temp_file.flush()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    // If file is not a WAV file, try to convert it
    let file_extension = Path::new(&filename).extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("").to_lowercase();
    
    let processing_path = if file_extension != "wav" {
        // Convert to WAV if not already in WAV format
        match web::block(move || {
            wav::convert_to_wav(&file_path, 1)
                .map_err(|e| e.to_string()) // Convert error to String to make it Send
        }).await {
            Ok(Ok(wav_path)) => wav_path,
            Ok(Err(err_msg)) => return Err(actix_web::error::ErrorInternalServerError(err_msg)),
            Err(e) => return Err(actix_web::error::ErrorInternalServerError(e))
        }
    } else {
        file_path
    };
    
    // Run find in a blocking task as it's CPU intensive
    let results = web::block(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            match shazam::find_matches_for_api(&processing_path).await {
                Ok(matches) => matches,
                Err(e) => {
                    println!("Error finding matches: {:?}", e);
                    Vec::new()
                }
            }
        })
    })
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().json(results))
}

// API endpoint for downloading songs
async fn api_download(url_data: web::Json<SpotifyUrl>) -> Result<impl Responder, Error> {
    web::block(move || {
        command_handlers::download(&url_data.url)
    })
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().body("Song download initiated"))
}

// API endpoint for saving songs
async fn api_save(mut payload: Multipart, query: web::Query<SaveOptions>) -> Result<impl Responder, Error> {
    let force = query.force.unwrap_or(false);
    let mut temp_file = NamedTempFile::new()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    let file_path = temp_file.path().to_string_lossy().to_string();
    
    // Process uploaded file
    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            temp_file.write_all(&data)
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        }
    }
    
    // Run save in a blocking task
    web::block(move || {
        command_handlers::save(&file_path, force)
    })
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().body("Song saved successfully"))
}

// API endpoint for erasing database
async fn api_erase() -> Result<impl Responder, Error> {
    web::block(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(command_handlers::erase("songs"))
    })
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().body("Database erased successfully"))
}

// Add this new endpoint function
async fn api_library() -> Result<impl Responder, Error> {
    let result = web::block(move || {
        // Create a runtime to handle async database operations
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let mut db_client = db::new_db_client().await
                .map_err(|e| format!("Failed to create DB client: {}", e))?;
            
            // Get all songs from database
            let songs = db_client.get_all_songs()
                .map_err(|e| format!("Failed to get songs: {}", e))?
                .into_iter()
                .map(|song| LibrarySong {
                    id: song.id,
                    name: song.name,
                    artist: song.artist,
                    youtube_id: song.youtube_id,
                    thumbnail_url: song.thumbnail_url,
                })
                .collect::<Vec<LibrarySong>>();
            
            db_client.close();
            
            Ok::<Vec<LibrarySong>, String>(songs)
        })
    })
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    match result {
        Ok(songs) => Ok(HttpResponse::Ok().json(songs)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Database error: {}", e))),
    }
}

// Configure and start the web server
pub async fn start_server(host: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000") // Next.js dev server
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_origin("https://your-frontend-app-name.vercel.app") // Production frontend
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec!["Content-Type", "Authorization"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/api/find", web::post().to(api_find))
            .route("/api/download", web::post().to(api_download))
            .route("/api/save", web::post().to(api_save))
            .route("/api/erase", web::post().to(api_erase))
            .route("/api/library", web::get().to(api_library)) // Add this line
    })
    .bind((host, port))?
    .run()
    .await
}