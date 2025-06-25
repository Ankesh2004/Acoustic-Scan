use std::error::Error;
use std::fmt;
use std::io::{self, Read};
use url::Url;
use std::str;
use std::time::Duration;

use reqwest::blocking::Client;
use reqwest::header::ACCEPT_LANGUAGE;
use serde_json::Value;
use url::form_urlencoded;

const DEVELOPER_KEY: &str = ""; // Insert your YouTube API key here if needed.
const DURATION_MATCH_THRESHOLD: i32 = 5;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub title: String,
    pub uploader: String,
    pub url: String,
    pub duration: String,
    pub id: String,
    pub live: bool,
    pub source_name: String,
    pub extra: Vec<String>,
}

/// Converts a duration string in format HH:MM:SS, MM:SS or SS into seconds.
pub fn convert_string_duration_to_seconds(duration_str: &str) -> i32 {
    let parts: Vec<&str> = duration_str.split(':').collect();
    match parts.len() {
        1 => parts[0].parse::<i32>().unwrap_or(0),
        2 => {
            let minutes = parts[0].parse::<i32>().unwrap_or(0);
            let seconds = parts[1].parse::<i32>().unwrap_or(0);
            minutes * 60 + seconds
        }
        3 => {
            let hours = parts[0].parse::<i32>().unwrap_or(0);
            let minutes = parts[1].parse::<i32>().unwrap_or(0);
            let seconds = parts[2].parse::<i32>().unwrap_or(0);
            hours * 3600 + minutes * 60 + seconds
        }
        _ => 0,
    }
}

/// Searches YouTube for a track matching the given Spotify track info and returns the video ID.
pub fn get_youtube_id(track: &crate::models::Track) -> Result<String, Box<dyn Error>> {
    let song_duration = track.duration; // in seconds
    let search_query = format!("'{}' {}", track.title, track.artist);
    let results = yt_search(&search_query, 10)?;
    if results.is_empty() {
        return Err(format!("no songs found for {}", search_query).into());
    }
    // Look for a result whose duration is within the allowed range.
    for result in results {
        let result_duration = convert_string_duration_to_seconds(&result.duration);
        let song_duration_i32 = song_duration as i32;
        if result_duration >= song_duration_i32 - DURATION_MATCH_THRESHOLD &&
           result_duration <= song_duration_i32 + DURATION_MATCH_THRESHOLD {
            return Ok(result.id);
        }
    }
    Err(format!("could not settle on a song from search result for: {}", search_query).into())
}

/// Searches YouTube by scraping the search results page and returns up to `limit` search results.
pub fn yt_search(search_term: &str, limit: usize) -> Result<Vec<SearchResult>, Box<dyn Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;
    let search_url = format!(
        "https://www.youtube.com/results?search_query={}",
        url::form_urlencoded::byte_serialize(search_term.as_bytes()).collect::<String>()
    );
    let req = client.get(&search_url)
        .header(ACCEPT_LANGUAGE, "en")
        .build()?;
    let resp = client.execute(req)?;
    if resp.status().as_u16() != 200 {
        return Err("failed to make a request to youtube".into());
    }
    let body = resp.text()?;
    // Attempt to locate the initial data JSON blob.
    let json_data = if let Some(idx) = body.find(r#"window["ytInitialData"] = "#) {
        let tail = &body[idx + r#"window["ytInitialData"] = "#.len()..];
        if let Some(end_idx) = tail.find(";</script>") {
            &tail[..end_idx]
        } else {
            return Err("invalid response from youtube (cannot find end marker)".into());
        }
    } else if let Some(idx) = body.find("var ytInitialData = ") {
        let tail = &body[idx + "var ytInitialData = ".len()..];
        if let Some(end_idx) = tail.find(";</script>") {
            &tail[..end_idx]
        } else {
            return Err("invalid response from youtube (cannot find end marker)".into());
        }
    } else {
        return Err("invalid response from youtube".into());
    };
    // Parse the JSON data.
    let data: Value = serde_json::from_str(json_data)?;
    // Navigate to the array of search result items.
    let items = data.pointer("/contents/twoColumnSearchResultsRenderer/primaryContents/sectionListRenderer/contents")
        .and_then(|v| v.as_array())
        .ok_or("failed to parse search results")?;
    // In some cases, the first element might be an ad carousel.
    let mut search_results = Vec::new();
    for section in items {
        if let Some(item_section) = section.get("itemSectionRenderer") {
            if let Some(contents) = item_section.get("contents").and_then(|v| v.as_array()) {
                for item in contents {
                    if let Some(video_renderer) = item.get("videoRenderer") {
                        // Extract video ID.
                        if let Some(video_id) = video_renderer.get("videoId").and_then(|v| v.as_str()) {
                            // Extract title.
                            let title = video_renderer.pointer("/title/runs/0/text")
                                .and_then(|v| v.as_str()).unwrap_or("").to_string();
                            // Extract uploader.
                            let uploader = video_renderer.pointer("/ownerText/runs/0/text")
                                .and_then(|v| v.as_str()).unwrap_or("").to_string();
                            // Extract duration (if available). If not, mark as live.
                            let (duration, live) = if let Some(dur) = video_renderer.get("lengthText")
                                .and_then(|v| v.get("simpleText"))
                                .and_then(|v| v.as_str()) {
                                    (dur.to_string(), false)
                                } else {
                                    ("".to_string(), true)
                                };
                            let url = format!("https://youtube.com/watch?v={}", video_id);
                            search_results.push(SearchResult {
                                title,
                                uploader,
                                duration,
                                id: video_id.to_string(),
                                url,
                                live,
                                source_name: "youtube".to_string(),
                                extra: Vec::new(),
                            });
                            if search_results.len() >= limit {
                                break;
                            }
                        }
                    }
                }
            }
        }
        if search_results.len() >= limit {
            break;
        }
    }
    Ok(search_results)
}

/// Uses the YouTube Data API to search for a video given a Spotify track.
/// (This is a placeholder implementation; you must add proper API key and error handling.)
pub fn get_youtube_id_with_api(sp_track: &crate::models::Track) -> Result<String, Box<dyn Error>> {
    // Using the YouTube API client is not as straightforward in Rust as in Go.
    // Here we assume you would use a suitable crate or HTTP requests.
    // This placeholder simply logs an error and returns an empty string.
    eprintln!("get_youtube_id_with_api is not implemented; returning empty string.");
    Ok(String::new())
}
