use std::error::Error;
use std::fmt;
use std::thread;
use std::time::Duration;

use regex::Regex;
use reqwest::blocking::{Client, Response};
use serde_json::Value;
use urlencoding::encode;

use crate::shazam;
use crate::utils;
use crate::models::Track; // Assumes your Track struct is defined in models

// Constants for endpoints.
const TOKEN_ENDPOINT: &str = "https://open.spotify.com/get_access_token?reason=transport&productType=web-player";
const TRACK_INITIAL_PATH: &str = "https://api-partner.spotify.com/pathfinder/v1/query?operationName=getTrack&variables=";
const PLAYLIST_INITIAL_PATH: &str = "https://api-partner.spotify.com/pathfinder/v1/query?operationName=fetchPlaylist&variables=";
const ALBUM_INITIAL_PATH: &str = "https://api-partner.spotify.com/pathfinder/v1/query?operationName=getAlbum&variables=";
const TRACK_END_PATH: &str = r#"{"persistedQuery":{"version":1,"sha256Hash":"e101aead6d78faa11d75bec5e36385a07b2f1c4a0420932d374d89ee17c70dd6"}}"#;
const PLAYLIST_END_PATH: &str = r#"{"persistedQuery":{"version":1,"sha256Hash":"b39f62e9b566aa849b1780927de1450f47e02c54abf1e66e513f96e849591e41"}}"#;
const ALBUM_END_PATH: &str = r#"{"persistedQuery":{"version":1,"sha256Hash":"46ae954ef2d2fe7732b4b2b4022157b2e18b7ea84f70591ceb164e4de1b5d5d3"}}"#;

/// Used for pagination when fetching resource information.
pub struct ResourceEndpoint {
    pub limit: i64,
    pub offset: i64,
    pub total_count: i64,
    pub requests: i64,
}

impl ResourceEndpoint {
    pub fn new(limit: i64) -> Self {
        ResourceEndpoint {
            limit,
            offset: 0,
            total_count: 0,
            requests: 0,
        }
    }
    pub fn paginate(&mut self) {
        self.offset += self.limit;
    }
}

/// Track representation (for Spotify).
/// Fields correspond to those needed from the JSON response.
#[derive(Clone, Debug)]
pub struct SpotifyTrack {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub artists: Vec<String>,
    pub duration: i32, // in seconds
}

impl SpotifyTrack {
    pub fn build_track(self) -> SpotifyTrack {
        self
    }
}

/// Retrieves an access token from Spotify.
fn access_token() -> Result<String, Box<dyn Error>> {
    let resp = reqwest::blocking::get(TOKEN_ENDPOINT)?;
    let body = resp.text()?;
    let v: Value = serde_json::from_str(&body)?;
    // Extract the "accessToken" field.
    if let Some(token) = v.get("accessToken").and_then(|t| t.as_str()) {
        Ok(token.to_string())
    } else {
        Err("accessToken not found".into())
    }
}

/// Makes a GET request to the given endpoint with an Authorization header.
fn request(endpoint: &str) -> Result<(u16, String), Box<dyn Error>> {
    let bearer = access_token()?;
    let client = Client::new();
    let resp = client
        .get(endpoint)
        .header("Authorization", format!("Bearer {}", bearer))
        .send()?;
    let status = resp.status().as_u16();
    let body = resp.text()?;
    Ok((status, body))
}

/// Extracts the ID from a Spotify URL.
fn get_id(url: &str) -> String {
    let parts: Vec<&str> = url.split('/').collect();
    if parts.len() > 4 {
        // Split the 5th part on '?' and return the first element.
        parts[4].split('?').next().unwrap_or("").to_string()
    } else {
        "".to_string()
    }
}

/// Checks if a given URL matches the specified regex pattern.
fn is_valid_pattern(url: &str, pattern: &str) -> bool {
    Regex::new(pattern)
        .map(|re| re.is_match(url))
        .unwrap_or(false)
}

/// Encodes a parameter value for URL usage.
fn encode_param(param: &str) -> String {
    encode(param).into_owned()
}

/// Retrieves track information from Spotify.
pub fn track_info(url: &str) -> Result<SpotifyTrack, Box<dyn Error>> {
    let track_pattern = r"^https:\/\/open\.spotify\.com\/track\/[a-zA-Z0-9]{22}\?si=[a-zA-Z0-9]{16}$";
    if !is_valid_pattern(url, track_pattern) {
        return Err("invalid track url".into());
    }
    let id = get_id(url);
    let query = format!(r#"{{"uri":"spotify:track:{}"}}"#, id);
    let endpoint_query = encode_param(&query);
    let endpoint = format!("{}{}&extensions={}", TRACK_INITIAL_PATH, endpoint_query, encode_param(TRACK_END_PATH));
    let (status, json_response) = request(&endpoint)?;
    if status != 200 {
        return Err(format!("received non-200 status code: {}", status).into());
    }
    let v: Value = serde_json::from_str(&json_response)?;
    // Extract the first artist.
    let mut all_artists = Vec::new();
    if let Some(first_artist) = v.pointer("/data/trackUnion/firstArtist/items/0/profile/name").and_then(|v| v.as_str()) {
        all_artists.push(first_artist.to_string());
    }
    // Extract additional artists.
    if let Some(artists_array) = v.pointer("/data/trackUnion/otherArtists/items").and_then(|v| v.as_array()) {
        for artist in artists_array {
            if let Some(name) = artist.pointer("/profile/name").and_then(|v| v.as_str()) {
                all_artists.push(name.to_string());
            }
        }
    }
    let duration_ms = v.pointer("/data/trackUnion/duration/totalMilliseconds").and_then(|v| v.as_i64()).unwrap_or(0);
    let duration_sec = (duration_ms / 1000) as i32;
    let track = SpotifyTrack {
        title: v.pointer("/data/trackUnion/name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        artist: v.pointer("/data/trackUnion/firstArtist/items/0/profile/name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        artists: all_artists,
        duration: duration_sec,
        album: v.pointer("/data/trackUnion/albumOfTrack/name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
    };
    Ok(track.build_track())
}

/// Retrieves playlist information (a list of tracks) from Spotify.
pub fn playlist_info(url: &str) -> Result<Vec<SpotifyTrack>, Box<dyn Error>> {
    let playlist_pattern = r"^https:\/\/open\.spotify\.com\/playlist\/[a-zA-Z0-9]{22}\?si=[a-zA-Z0-9]{16}$";
    if !is_valid_pattern(url, playlist_pattern) {
        return Err("invalid playlist url".into());
    }
    let total_count = "data.playlistV2.content.totalCount";
    let items_array = "data.playlistV2.content.items";
    resource_info(url, "playlist", total_count, items_array)
}

/// Retrieves album information (a list of tracks) from Spotify.
pub fn album_info(url: &str) -> Result<Vec<SpotifyTrack>, Box<dyn Error>> {
    let album_pattern = r"^https:\/\/open\.spotify\.com\/album\/[a-zA-Z0-9-]{22}\?si=[a-zA-Z0-9_-]{22}$";
    if !is_valid_pattern(url, album_pattern) {
        return Err("invalid album url".into());
    }
    let total_count = "data.albumUnion.discs.items.0.tracks.totalCount";
    let items_array = "data.albumUnion.discs.items";
    resource_info(url, "album", total_count, items_array)
}

/// Fetches resource information (for playlists or albums) and returns a vector of tracks.
fn resource_info(url: &str, resource_type: &str, total_count_path: &str, _items_array: &str) -> Result<Vec<SpotifyTrack>, Box<dyn Error>> {
    let id = get_id(url);
    let mut endpoint_conf = ResourceEndpoint::new(400);
    let json_response = json_list(resource_type, &id, endpoint_conf.offset, endpoint_conf.limit)?;
    let total = serde_json::from_str::<Value>(&json_response)?
        .pointer(total_count_path)
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    endpoint_conf.total_count = total;
    if endpoint_conf.total_count < 1 {
        return Err("hum, there are no tracks".into());
    }
    // Get resource name (playlist or album).
    let name = if resource_type == "playlist" {
        serde_json::from_str::<Value>(&json_response)?
            .pointer("/data/playlistV2/name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    } else {
        serde_json::from_str::<Value>(&json_response)?
            .pointer("/data/albumUnion/name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    };
    println!("Collecting tracks from '{}'...", name);
    thread::sleep(Duration::from_secs(1));
    endpoint_conf.requests = ((endpoint_conf.total_count as f64) / (endpoint_conf.limit as f64)).ceil() as i64;
    let mut tracks = process_items(&json_response, resource_type);
    for _i in 1..endpoint_conf.requests {
        endpoint_conf.paginate();
        let json_response = json_list(resource_type, &id, endpoint_conf.offset, endpoint_conf.limit)?;
        tracks.append(&mut process_items(&json_response, resource_type));
    }
    println!("Tracks collected: {}", tracks.len());
    Ok(tracks)
}

/// Constructs the proper endpoint URL and fetches JSON from Spotify.
fn json_list(resource_type: &str, id: &str, offset: i64, limit: i64) -> Result<String, Box<dyn Error>> {
    let endpoint = if resource_type == "playlist" {
        let query = format!(r#"{{"uri":"spotify:playlist:{}","offset":{},"limit":{}}}"#, id, offset, limit);
        format!("{}{}&extensions={}", PLAYLIST_INITIAL_PATH, encode_param(&query), encode_param(PLAYLIST_END_PATH))
    } else {
        let query = format!(r#"{{"uri":"spotify:album:{}","locale":"","offset":{},"limit":{}}}"#, id, offset, limit);
        format!("{}{}&extensions={}", ALBUM_INITIAL_PATH, encode_param(&query), encode_param(ALBUM_END_PATH))
    };

    let (status, json_response) = request(&endpoint)?;
    if status != 200 {
        return Err(format!("received non-200 status code: {}", status).into());
    }
    Ok(json_response)
}

/// Processes items from the JSON response and returns a vector of SpotifyTrack.
fn process_items(json_response: &str, resource_type: &str) -> Vec<SpotifyTrack> {
    // Define JSON pointers for different resource types.
    let (item_list, song_title, artist_name, album_name, duration_path) = if resource_type == "playlist" {
        (
            "/data/playlistV2/content/items",
            "itemV2.data.name",
            "itemV2.data.artists.items.0.profile.name",
            "itemV2.data.albumOfTrack.name",
            "itemV2.data.trackDuration.totalMilliseconds",
        )
    } else {
        (
            "/data/albumUnion/tracks/items",
            "track.name",
            "track.artists.items.0.profile.name",
            "/data/albumUnion/name", // For album, the album name is at a higher level.
            "track.duration.totalMilliseconds",
        )
    };

    let v: Value = match serde_json::from_str(json_response) {
        Ok(val) => val,
        Err(_) => return vec![],
    };

    let empty_vec = Vec::new();
    let items = v.pointer(item_list).and_then(|v| v.as_array()).unwrap_or(&empty_vec);
    let mut tracks = Vec::new();

    for item in items {
        let duration_ms = item.pointer(duration_path).and_then(|v| v.as_i64()).unwrap_or(0);
        let duration_sec = (duration_ms / 1000) as i32;
        let title = item.pointer(song_title).and_then(|v| v.as_str()).unwrap_or("").to_string();
        let artist = item.pointer(artist_name).and_then(|v| v.as_str()).unwrap_or("").to_string();
        let album = if resource_type == "playlist" {
            item.pointer(album_name).and_then(|v| v.as_str()).unwrap_or("").to_string()
        } else {
            // For album, use the album name from the root.
            v.pointer(album_name).and_then(|v| v.as_str()).unwrap_or("").to_string()
        };

        let track = SpotifyTrack {
            title,
            artist,
            album,
            artists: vec![], // You could add more detailed artist info if needed.
            duration: duration_sec,
        };
        tracks.push(track.build_track());
    }
    tracks
}
