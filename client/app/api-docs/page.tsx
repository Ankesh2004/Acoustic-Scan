import { Navbar } from "@/components/navbar";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  Code,
  FileJson,
  Fingerprint,
  AudioWaveformIcon as Waveform,
  Library,
  Download,
} from "lucide-react";
import { CopyButton } from "@/components/copy-button";

export default function ApiDocsPage() {
  return (
    <div className="flex min-h-screen flex-col">
      <Navbar />
      <main className="flex-1 container py-8 md:py-12">
        <div className="mx-auto max-w-5xl space-y-8">
          <div className="space-y-2">
            <h1 className="text-3xl font-bold tracking-tight">
              API Documentation
            </h1>
            <p className="text-muted-foreground">
              Learn how to integrate with the Acousti-Scan API
            </p>
          </div>

          <Tabs defaultValue="overview" className="space-y-8">
            <TabsList className="grid w-full grid-cols-5">
              <TabsTrigger value="overview">Overview</TabsTrigger>
              <TabsTrigger value="identify">Identify</TabsTrigger>
              <TabsTrigger value="contribute">Contribute</TabsTrigger>
              <TabsTrigger value="library">Library</TabsTrigger>
              <TabsTrigger value="examples">Examples</TabsTrigger>
            </TabsList>

            <TabsContent
              value="overview"
              className="space-y-6 animate-slide-up"
            >
              <Card>
                <CardHeader>
                  <CardTitle>Getting Started</CardTitle>
                  <CardDescription>
                    Learn the basics of the Acousti-Scan API
                  </CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                  <div className="flex">
                    <div className="mr-6 flex flex-col items-center">
                      <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                        <FileJson className="h-5 w-5 text-primary" />
                      </div>
                      <div className="mt-2 h-full w-px bg-border" />
                    </div>
                    <div className="space-y-2 pb-8">
                      <h3 className="text-lg font-medium">Base URL</h3>
                      <p className="text-muted-foreground">
                        All API requests should be made to the following base
                        URL:
                      </p>
                      <div className="mt-2 rounded-md bg-muted p-4 overflow-x-auto relative group">
                        <CopyButton
                          value="https://acoustic-scan.onrender.com"
                          className="absolute right-2 top-2 opacity-0 group-hover:opacity-100 transition-opacity"
                        />
                        <code className="text-sm">{`https://acoustic-scan.onrender.com`}</code>
                      </div>
                    </div>
                  </div>

                  <div className="flex">
                    <div className="mr-6 flex flex-col items-center">
                      <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                        <Code className="h-5 w-5 text-primary" />
                      </div>
                      <div className="mt-2 h-full w-px bg-border" />
                    </div>
                    <div className="space-y-2 pb-8">
                      <h3 className="text-lg font-medium">Content Types</h3>
                      <p className="text-muted-foreground">
                        The API accepts multipart/form-data for file uploads and
                        returns JSON responses.
                      </p>
                      <div className="mt-2 rounded-md bg-muted p-4 overflow-x-auto relative group">
                        <CopyButton
                          value={`Request: multipart/form-data
Response: application/json`}
                          className="absolute right-2 top-2 opacity-0 group-hover:opacity-100 transition-opacity"
                        />
                        <code className="text-sm">{`Request: multipart/form-data
Response: application/json`}</code>
                      </div>
                    </div>
                  </div>

                  <div className="flex">
                    <div className="mr-6 flex flex-col items-center">
                      <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                        <Waveform className="h-5 w-5 text-primary" />
                      </div>
                    </div>
                    <div className="space-y-2">
                      <h3 className="text-lg font-medium">
                        Supported Audio Formats
                      </h3>
                      <p className="text-muted-foreground">
                        The API supports common audio formats including MP3,
                        WAV, FLAC, and other formats supported by FFmpeg.
                      </p>
                    </div>
                  </div>
                </CardContent>
              </Card>
            </TabsContent>

            <TabsContent
              value="identify"
              className="space-y-6 animate-slide-up"
            >
              <Card>
                <CardHeader>
                  <CardTitle>Song Identification API</CardTitle>
                  <CardDescription>
                    Identify songs using audio fingerprinting technology
                  </CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                  <div className="flex">
                    <div className="mr-6 flex flex-col items-center">
                      <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                        <Fingerprint className="h-5 w-5 text-primary" />
                      </div>
                      <div className="mt-2 h-full w-px bg-border" />
                    </div>
                    <div className="space-y-2 pb-8">
                      <h3 className="text-lg font-medium">Endpoint</h3>
                      <p className="text-muted-foreground">
                        Send a POST request with an audio file to identify a
                        song using acoustic fingerprinting.
                      </p>
                      <div className="mt-2 rounded-md bg-muted p-4 overflow-x-auto relative group">
                        <CopyButton
                          value="POST /api/find"
                          className="absolute right-2 top-2 opacity-0 group-hover:opacity-100 transition-opacity"
                        />
                        <code className="text-sm">{`POST /api/find`}</code>
                      </div>
                    </div>
                  </div>

                  <div className="flex">
                    <div className="mr-6 flex flex-col items-center">
                      <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                        <FileJson className="h-5 w-5 text-primary" />
                      </div>
                      <div className="mt-2 h-full w-px bg-border" />
                    </div>
                    <div className="space-y-2 pb-8">
                      <h3 className="text-lg font-medium">Request Format</h3>
                      <p className="text-muted-foreground">
                        Upload an audio file using multipart/form-data. The
                        system will analyze the audio and return matching songs.
                      </p>
                      <div className="mt-2 rounded-md bg-muted p-4 overflow-x-auto relative group">
                        <CopyButton
                          value={`Content-Type: multipart/form-data

file: [audio file binary data]`}
                          className="absolute right-2 top-2 opacity-0 group-hover:opacity-100 transition-opacity"
                        />
                        <code className="text-sm">
                          {`Content-Type: multipart/form-data

file: [audio file binary data]`}
                        </code>
                      </div>
                    </div>
                  </div>

                  <div className="flex">
                    <div className="mr-6 flex flex-col items-center">
                      <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                        <Code className="h-5 w-5 text-primary" />
                      </div>
                    </div>
                    <div className="space-y-2">
                      <h3 className="text-lg font-medium">Response Format</h3>
                      <p className="text-muted-foreground">
                        Returns an array of matching songs with confidence
                        scores, timestamps, and YouTube links.
                      </p>
                      <div className="mt-2 rounded-md bg-muted p-4 overflow-x-auto relative group">
                        <CopyButton
                          value={`[
  {
    "song_id": 123,
    "song_title": "Bohemian Rhapsody",
    "song_artist": "Queen",
    "youtube_id": "fJ9rUzIMcZQ",
    "timestamp": 45000,
    "score": 892.5
  },
  ...
]`}
                          className="absolute right-2 top-2 opacity-0 group-hover:opacity-100 transition-opacity"
                        />
                        <code className="text-sm">
                          {`[
  {
    "song_id": 123,
    "song_title": "Bohemian Rhapsody",
    "song_artist": "Queen",
    "youtube_id": "fJ9rUzIMcZQ",
    "timestamp": 45000,
    "score": 892.5
  },
  ...
]`}
                        </code>
                      </div>
                    </div>
                  </div>
                </CardContent>
              </Card>
            </TabsContent>

            <TabsContent
              value="contribute"
              className="space-y-6 animate-slide-up"
            >
              <Card>
                <CardHeader>
                  <CardTitle>Song Contribution API</CardTitle>
                  <CardDescription>
                    Add new songs to the Acousti-Scan database
                  </CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                  <div className="flex">
                    <div className="mr-6 flex flex-col items-center">
                      <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                        <Download className="h-5 w-5 text-primary" />
                      </div>
                      <div className="mt-2 h-full w-px bg-border" />
                    </div>
                    <div className="space-y-2 pb-8">
                      <h3 className="text-lg font-medium">Download Endpoint</h3>
                      <p className="text-muted-foreground">
                        Download and add a song to the database using a Spotify
                        URL.
                      </p>
                      <div className="mt-2 rounded-md bg-muted p-4 overflow-x-auto relative group">
                        <CopyButton
                          value="POST /api/download"
                          className="absolute right-2 top-2 opacity-0 group-hover:opacity-100 transition-opacity"
                        />
                        <code className="text-sm">{`POST /api/download`}</code>
                      </div>
                    </div>
                  </div>

                  <div className="flex">
                    <div className="mr-6 flex flex-col items-center">
                      <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                        <FileJson className="h-5 w-5 text-primary" />
                      </div>
                      <div className="mt-2 h-full w-px bg-border" />
                    </div>
                    <div className="space-y-2 pb-8">
                      <h3 className="text-lg font-medium">
                        Download Request Format
                      </h3>
                      <p className="text-muted-foreground">
                        Send a JSON request with a Spotify track URL to download
                        and add the song.
                      </p>
                      <div className="mt-2 rounded-md bg-muted p-4 overflow-x-auto relative group">
                        <CopyButton
                          value={`Content-Type: application/json

{
  "spotify_url": "https://open.spotify.com/track/4uLU6hMCjMI75M1A2tKUQC"
}`}
                          className="absolute right-2 top-2 opacity-0 group-hover:opacity-100 transition-opacity"
                        />
                        <code className="text-sm">
                          {`Content-Type: application/json

{
  "spotify_url": "https://open.spotify.com/track/4uLU6hMCjMI75M1A2tKUQC"
}`}
                        </code>
                      </div>
                    </div>
                  </div>

                  <div className="flex">
                    <div className="mr-6 flex flex-col items-center">
                      <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                        <Fingerprint className="h-5 w-5 text-primary" />
                      </div>
                      <div className="mt-2 h-full w-px bg-border" />
                    </div>
                    <div className="space-y-2 pb-8">
                      <h3 className="text-lg font-medium">Upload Endpoint</h3>
                      <p className="text-muted-foreground">
                        Upload an audio file with metadata to add it directly to
                        the database.
                      </p>
                      <div className="mt-2 rounded-md bg-muted p-4 overflow-x-auto relative group">
                        <CopyButton
                          value="POST /api/upload"
                          className="absolute right-2 top-2 opacity-0 group-hover:opacity-100 transition-opacity"
                        />
                        <code className="text-sm">{`POST /api/upload`}</code>
                      </div>
                    </div>
                  </div>

                  <div className="flex">
                    <div className="mr-6 flex flex-col items-center">
                      <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                        <Code className="h-5 w-5 text-primary" />
                      </div>
                    </div>
                    <div className="space-y-2">
                      <h3 className="text-lg font-medium">
                        Upload Request Format
                      </h3>
                      <p className="text-muted-foreground">
                        Upload an audio file along with song metadata using
                        multipart/form-data.
                      </p>
                      <div className="mt-2 rounded-md bg-muted p-4 overflow-x-auto relative group">
                        <CopyButton
                          value={`Content-Type: multipart/form-data

file: [audio file binary data]
name: "Song Name"
artist: "Artist Name"
youtube_id: "dQw4w9WgXcQ"`}
                          className="absolute right-2 top-2 opacity-0 group-hover:opacity-100 transition-opacity"
                        />
                        <code className="text-sm">
                          {`Content-Type: multipart/form-data

file: [audio file binary data]
name: "Song Name"
artist: "Artist Name"
youtube_id: "dQw4w9WgXcQ"`}
                        </code>
                      </div>
                    </div>
                  </div>
                </CardContent>
              </Card>
            </TabsContent>

            <TabsContent value="library" className="space-y-6 animate-slide-up">
              <Card>
                <CardHeader>
                  <CardTitle>Library API</CardTitle>
                  <CardDescription>
                    Access and manage the song library
                  </CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                  <div className="flex">
                    <div className="mr-6 flex flex-col items-center">
                      <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                        <Library className="h-5 w-5 text-primary" />
                      </div>
                      <div className="mt-2 h-full w-px bg-border" />
                    </div>
                    <div className="space-y-2 pb-8">
                      <h3 className="text-lg font-medium">Endpoint</h3>
                      <p className="text-muted-foreground">
                        Retrieve all songs in the database library.
                      </p>
                      <div className="mt-2 rounded-md bg-muted p-4 overflow-x-auto relative group">
                        <CopyButton
                          value="GET /api/library"
                          className="absolute right-2 top-2 opacity-0 group-hover:opacity-100 transition-opacity"
                        />
                        <code className="text-sm">{`GET /api/library`}</code>
                      </div>
                    </div>
                  </div>

                  <div className="flex">
                    <div className="mr-6 flex flex-col items-center">
                      <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                        <Code className="h-5 w-5 text-primary" />
                      </div>
                    </div>
                    <div className="space-y-2">
                      <h3 className="text-lg font-medium">Response Format</h3>
                      <p className="text-muted-foreground">
                        Returns an array of all songs in the library with their
                        metadata and thumbnails.
                      </p>
                      <div className="mt-2 rounded-md bg-muted p-4 overflow-x-auto relative group">
                        <CopyButton
                          value={`[
  {
    "id": 1,
    "name": "Bohemian Rhapsody",
    "artist": "Queen",
    "youtube_id": "fJ9rUzIMcZQ",
    "thumbnail_url": "https://img.youtube.com/vi/fJ9rUzIMcZQ/maxresdefault.jpg"
  },
  {
    "id": 2,
    "name": "Imagine",
    "artist": "John Lennon",
    "youtube_id": "YkgkThdzX-8",
    "thumbnail_url": "https://img.youtube.com/vi/YkgkThdzX-8/maxresdefault.jpg"
  },
  ...
]`}
                          className="absolute right-2 top-2 opacity-0 group-hover:opacity-100 transition-opacity"
                        />
                        <code className="text-sm">
                          {`[
  {
    "id": 1,
    "name": "Bohemian Rhapsody",
    "artist": "Queen",
    "youtube_id": "fJ9rUzIMcZQ",
    "thumbnail_url": "https://img.youtube.com/vi/fJ9rUzIMcZQ/maxresdefault.jpg"
  },
  {
    "id": 2,
    "name": "Imagine",
    "artist": "John Lennon",
    "youtube_id": "YkgkThdzX-8",
    "thumbnail_url": "https://img.youtube.com/vi/YkgkThdzX-8/maxresdefault.jpg"
  },
  ...
]`}
                        </code>
                      </div>
                    </div>
                  </div>
                </CardContent>
              </Card>
            </TabsContent>

            <TabsContent
              value="examples"
              className="space-y-6 animate-slide-up"
            >
              <Card>
                <CardHeader>
                  <CardTitle>Code Examples</CardTitle>
                  <CardDescription>
                    Examples of how to use the Acousti-Scan API in different
                    languages
                  </CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                  <div className="flex">
                    <div className="mr-6 flex flex-col items-center">
                      <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                        <Code className="h-5 w-5 text-primary" />
                      </div>
                      <div className="mt-2 h-full w-px bg-border" />
                    </div>
                    <div className="space-y-2 pb-8">
                      <h3 className="text-lg font-medium">
                        JavaScript/TypeScript
                      </h3>
                      <p className="text-muted-foreground">
                        Example of how to identify a song using the browser's
                        fetch API.
                      </p>
                      <div className="mt-2 rounded-md bg-muted p-4 overflow-x-auto relative group">
                        <CopyButton
                          value={`// Identify a song
async function identifySong(audioFile: File) {
  const formData = new FormData();
  formData.append('file', audioFile);

  const response = await fetch('https://acoustic-scan.onrender.com/api/find', {
    method: 'POST',
    body: formData
  });

  if (!response.ok) {
    throw new Error('Failed to identify song');
  }

  const matches = await response.json();
  return matches;
}

// Get library
async function getLibrary() {
  const response = await fetch('https://acoustic-scan.onrender.com/api/library');
  const library = await response.json();
  return library;
}

// Download from Spotify
async function downloadFromSpotify(spotifyUrl: string) {
  const response = await fetch('https://acoustic-scan.onrender.com/api/download', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ spotify_url: spotifyUrl })
  });

  return response.json();
}`}
                          className="absolute right-2 top-2 opacity-0 group-hover:opacity-100 transition-opacity"
                        />
                        <code className="text-sm">
                          {`// Identify a song
async function identifySong(audioFile: File) {
  const formData = new FormData();
  formData.append('file', audioFile);

  const response = await fetch('https://acoustic-scan.onrender.com/api/find', {
    method: 'POST',
    body: formData
  });

  if (!response.ok) {
    throw new Error('Failed to identify song');
  }

  const matches = await response.json();
  return matches;
}

// Get library
async function getLibrary() {
  const response = await fetch('https://acoustic-scan.onrender.com/api/library');
  const library = await response.json();
  return library;
}

// Download from Spotify
async function downloadFromSpotify(spotifyUrl: string) {
  const response = await fetch('https://acoustic-scan.onrender.com/api/download', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ spotify_url: spotifyUrl })
  });

  return response.json();
}`}
                        </code>
                      </div>
                    </div>
                  </div>

                  <div className="flex">
                    <div className="mr-6 flex flex-col items-center">
                      <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                        <Code className="h-5 w-5 text-primary" />
                      </div>
                      <div className="mt-2 h-full w-px bg-border" />
                    </div>
                    <div className="space-y-2 pb-8">
                      <h3 className="text-lg font-medium">Python</h3>
                      <p className="text-muted-foreground">
                        Example using Python with the requests library.
                      </p>
                      <div className="mt-2 rounded-md bg-muted p-4 overflow-x-auto relative group">
                        <CopyButton
                          value={`import requests
import json

# Identify a song
def identify_song(file_path):
    url = 'https://acoustic-scan.onrender.com/api/find'
    
    with open(file_path, 'rb') as f:
        files = {'file': f}
        response = requests.post(url, files=files)
    
    response.raise_for_status()
    return response.json()

# Get library
def get_library():
    url = 'https://acoustic-scan.onrender.com/api/library'
    response = requests.get(url)
    response.raise_for_status()
    return response.json()

# Download from Spotify
def download_from_spotify(spotify_url):
    url = 'https://acoustic-scan.onrender.com/api/download'
    data = {'spotify_url': spotify_url}
    
    response = requests.post(url, json=data)
    response.raise_for_status()
    return response.json()

# Usage
matches = identify_song('path/to/audio.mp3')
library = get_library()
result = download_from_spotify('https://open.spotify.com/track/...')`}
                          className="absolute right-2 top-2 opacity-0 group-hover:opacity-100 transition-opacity"
                        />
                        <code className="text-sm">
                          {`import requests
import json

# Identify a song
def identify_song(file_path):
    url = 'https://acoustic-scan.onrender.com/api/find'
    
    with open(file_path, 'rb') as f:
        files = {'file': f}
        response = requests.post(url, files=files)
    
    response.raise_for_status()
    return response.json()

# Get library
def get_library():
    url = 'https://acoustic-scan.onrender.com/api/library'
    response = requests.get(url)
    response.raise_for_status()
    return response.json()

# Download from Spotify
def download_from_spotify(spotify_url):
    url = 'https://acoustic-scan.onrender.com/api/download'
    data = {'spotify_url': spotify_url}
    
    response = requests.post(url, json=data)
    response.raise_for_status()
    return response.json()

# Usage
matches = identify_song('path/to/audio.mp3')
library = get_library()
result = download_from_spotify('https://open.spotify.com/track/...')`}
                        </code>
                      </div>
                    </div>
                  </div>

                  <div className="flex">
                    <div className="mr-6 flex flex-col items-center">
                      <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                        <Code className="h-5 w-5 text-primary" />
                      </div>
                    </div>
                    <div className="space-y-2">
                      <h3 className="text-lg font-medium">Rust</h3>
                      <p className="text-muted-foreground">
                        Example using Rust with reqwest and tokio for async
                        operations.
                      </p>
                      <div className="mt-2 rounded-md bg-muted p-4 overflow-x-auto relative group">
                        <CopyButton
                          value={`use reqwest::multipart;
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Identify a song
    let file = tokio::fs::File::open("path/to/audio.mp3").await?;
    let file_part = multipart::Part::stream(file)
        .file_name("audio.mp3")
        .mime_str("audio/mpeg")?;
    
    let form = multipart::Form::new()
        .part("file", file_part);
    
    let client = reqwest::Client::new();
    let response = client
        .post("https://acoustic-scan.onrender.com/api/find")
        .multipart(form)
        .send()
        .await?;
    
    let matches: serde_json::Value = response.json().await?;
    println!("Matches: {:#?}", matches);
    
    // Get library
    let library_response = client
        .get("https://acoustic-scan.onrender.com/api/library")
        .send()
        .await?;
    
    let library: serde_json::Value = library_response.json().await?;
    println!("Library: {:#?}", library);
    
    // Download from Spotify
    let download_response = client
        .post("https://acoustic-scan.onrender.com/api/download")
        .json(&json!({
            "spotify_url": "https://open.spotify.com/track/..."
        }))
        .send()
        .await?;
    
    let result: serde_json::Value = download_response.json().await?;
    println!("Download result: {:#?}", result);
    
    Ok(())
}`}
                          className="absolute right-2 top-2 opacity-0 group-hover:opacity-100 transition-opacity"
                        />
                        <code className="text-sm">
                          {`use reqwest::multipart;
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Identify a song
    let file = tokio::fs::File::open("path/to/audio.mp3").await?;
    let file_part = multipart::Part::stream(file)
        .file_name("audio.mp3")
        .mime_str("audio/mpeg")?;
    
    let form = multipart::Form::new()
        .part("file", file_part);
    
    let client = reqwest::Client::new();
    let response = client
        .post("https://acoustic-scan.onrender.com/api/find")
        .multipart(form)
        .send()
        .await?;
    
    let matches: serde_json::Value = response.json().await?;
    println!("Matches: {:#?}", matches);
    
    // Get library
    let library_response = client
        .get("https://acoustic-scan.onrender.com/api/library")
        .send()
        .await?;
    
    let library: serde_json::Value = library_response.json().await?;
    println!("Library: {:#?}", library);
    
    // Download from Spotify
    let download_response = client
        .post("https://acoustic-scan.onrender.com/api/download")
        .json(&json!({
            "spotify_url": "https://open.spotify.com/track/..."
        }))
        .send()
        .await?;
    
    let result: serde_json::Value = download_response.json().await?;
    println!("Download result: {:#?}", result);
    
    Ok(())
}`}
                        </code>
                      </div>
                    </div>
                  </div>
                </CardContent>
              </Card>
            </TabsContent>
          </Tabs>
        </div>
      </main>
    </div>
  );
}
