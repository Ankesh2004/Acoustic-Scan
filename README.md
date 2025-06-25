# Acousti-Scan RS

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/yourusername/acousti-scan-rs)
[![API Docs](https://img.shields.io/badge/API-docs-blue.svg)](http://localhost:8080/api)

A powerful, Rust-based audio fingerprinting and recognition system that can identify songs, build music libraries, and provide real-time audio analysis capabilities. Built with modern web technologies and a robust Rust backend.

![Acousti-Scan Banner](https://github.com/user-attachments/assets/4c630c16-eaf1-4b8a-91a8-5db02463c5af)


## 🎵 Overview

Acousti-Scan RS is a comprehensive audio fingerprinting solution that combines the performance of Rust with the flexibility of a modern web interface. The system can analyze audio files, create unique fingerprints, and match unknown audio against a database of known songs. It features both a web-based client interface and a REST API for programmatic access.

The project implements Shazam-like functionality using advanced signal processing techniques including spectrograms, peak detection, and hash-based fingerprinting algorithms.

## ✨ Features

### Core Functionality
- **🎯 Audio Identification**: Identify songs from audio files or recordings with high accuracy
- **📚 Library Management**: Build and maintain a comprehensive music database
- **🎤 Real-time Recognition**: Process live audio input for instant song identification
- **📊 Audio Analysis**: Generate spectrograms and analyze audio characteristics
- **🔗 YouTube Integration**: Automatic metadata fetching and YouTube ID resolution
- **☁️ Spotify Integration**: Download and process songs directly from Spotify URLs

### Technical Features
- **⚡ High Performance**: Rust-powered backend for optimal processing speed
- **🌐 REST API**: Complete HTTP API for integration with external applications
- **💾 SQLite Database**: Lightweight, embedded database for fingerprint storage
- **🎨 Modern UI**: React-based web interface with responsive design
- **📱 Mobile Support**: Touch-friendly interface optimized for mobile devices
- **🔄 Real-time Processing**: WebSocket support for live audio streaming

### Audio Processing
- **FFT Analysis**: Fast Fourier Transform for frequency domain analysis
- **Peak Detection**: Advanced algorithms for identifying spectral peaks
- **Fingerprint Generation**: Robust hash-based audio fingerprinting
- **Multi-format Support**: MP3, WAV, FLAC, and other common audio formats
- **Noise Resilience**: Handles compressed and noisy audio inputs

## 🛠️ Tech Stack

### Backend (Rust)
- **[Tokio](https://tokio.rs/)** - Asynchronous runtime
- **[Actix Web](https://actix.rs/)** - HTTP server framework
- **[SQLite](https://sqlite.org/)** - Embedded database
- **[FFmpeg](https://ffmpeg.org/)** - Audio processing and conversion
- **[Serde](https://serde.rs/)** - Serialization framework
- **[Reqwest](https://github.com/seanmonstar/reqwest)** - HTTP client

### Frontend (Next.js)
- **[Next.js 14](https://nextjs.org/)** - React framework
- **[TypeScript](https://www.typescriptlang.org/)** - Type-safe JavaScript
- **[Tailwind CSS](https://tailwindcss.com/)** - Utility-first CSS framework
- **[Radix UI](https://www.radix-ui.com/)** - Headless UI components
- **[Lucide Icons](https://lucide.dev/)** - Modern icon library

### Audio Processing
- **Custom Shazam Algorithm** - Proprietary fingerprinting implementation
- **Spectrogram Analysis** - Time-frequency domain processing
- **Peak Extraction** - Constellation map generation
- **Hash Fingerprinting** - Robust audio signatures

## 📦 Installation

### Prerequisites

Ensure you have the following installed:
- **Rust** (1.70 or higher) - [Install Rust](https://rustup.rs/)
- **Node.js** (18 or higher) - [Install Node.js](https://nodejs.org/)
- **FFmpeg** - Audio processing library
- **SQLite** - Database engine

#### Install FFmpeg

**macOS:**
```bash
brew install ffmpeg
```

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install ffmpeg
```

**Windows:** Download from [FFmpeg official website](https://ffmpeg.org/)

### Clone and Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/acousti-scan-rs.git
cd acousti-scan-rs

# Build the Rust backend
cargo build --release

# Setup the frontend
cd client
npm install
# or
pnpm install
```

### Database Setup
The application will automatically create the SQLite database on first run:

```bash
# Create necessary directories
mkdir -p songs tmp

# Initialize the database (automatic on first API call)
cargo run --release api-server
```

## 🚀 Usage

### Starting the Application
1. Start the Backend API Server

```bash
# Start the Rust API server (default: localhost:8080)
cargo run --release api-server

# Or specify custom host and port
cargo run --release api-server 0.0.0.0 3001
```

2. Start the Frontend Development Server

```bash
cd client
npm run dev
# or
pnpm dev
```

The web interface will be available at http://localhost:3000

![Homepage](https://github.com/user-attachments/assets/f890ca58-dfb9-4721-a29f-ac0593584c06)

### Command Line Interface
The application provides several CLI commands for direct interaction:

#### Identify a Song

```bash
# Identify a song from an audio file
cargo run --release find path/to/audio.wav
```

![CLI Identification](https://github.com/user-attachments/assets/a1d5178b-5f85-431f-9c59-e9c9fbceeabc)

#### Add Songs to Database

```bash
# Add a single song (with automatic YouTube ID lookup)
cargo run --release save path/to/song.mp3

# Add a song without YouTube ID requirement
cargo run --release save -f path/to/song.mp3

# Add all songs in a directory
cargo run --release save path/to/music/directory/
```

#### Download from Spotify

```bash
# Download and add a song from Spotify URL
cargo run --release download "https://open.spotify.com/track/4uLU6hMCjMI75M1A2tKUQC"
```

#### Database Management

```bash
# Clear the entire database
cargo run --release erase
```

### Web Interface Usage
1. Song Identification
  - Navigate to the Scan page
    ![Scan Inteface](https://github.com/user-attachments/assets/b2c4bfb4-e62a-4988-9702-7ef926011318)
    
  - Upload an audio file or record directly in the browser
    ![Upload Song](https://github.com/user-attachments/assets/413eb045-ce0a-4919-8044-47e091646499)

  - View identification results with confidence scores and YouTube links
    ![Matches-Found](https://github.com/user-attachments/assets/91c8d00e-526b-4a4f-83e0-2ddcaa53beb2)


2. Library Management
  - Access the Library tab to view all stored songs
  - Search and filter through your music collection
  - Play preview clips and access YouTube links
    ![Library Interface](https://github.com/user-attachments/assets/003a7e8a-9ad1-47ec-bbe7-19f4cdca5730)

3. Contributing Songs
  - Use the Contribute tab to add new songs
  - Upload audio files or provide Spotify URLs
  - System automatically extracts metadata and creates fingerprints
   ![Contribute Interface](https://github.com/user-attachments/assets/155df6af-09c1-4496-bfae-a4aae3929831)

### Audio Processing Pipeline
The system follows this processing pipeline:

1. Audio Input → File upload or microphone recording
2. Format Conversion → Convert to WAV using FFmpeg
3. Spectrogram Generation → Create time-frequency representation
4. Peak Detection → Identify prominent frequency peaks
5. Fingerprint Creation → Generate hash-based signatures
6. Database Storage/Matching → Store or compare against existing fingerprints

![image](https://github.com/user-attachments/assets/33603d19-0cb3-457f-b324-207945ab4c84)

![image](https://github.com/user-attachments/assets/10596479-3d04-4793-9f9c-2f84cfe93ac9)


## 📡 API Reference
The system provides a comprehensive REST API for programmatic access:

### Base URL
http://localhost:8080/api

### Endpoints

#### Identify Song

```
POST /api/find
Content-Type: multipart/form-data

# Form data:
file: [audio file binary]
```

Response:
```json
[
  {
   "song_id": 123,
   "song_title": "Bohemian Rhapsody",
   "song_artist": "Queen",
   "youtube_id": "fJ9rUzIMcZQ",
   "timestamp": 45000,
   "score": 892.5
  }
]
```

#### Get Library
```
GET /api/library
```

Response:
```json
[
  {
   "id": 1,
   "name": "Bohemian Rhapsody",
   "artist": "Queen",
   "youtube_id": "fJ9rUzIMcZQ",
   "thumbnail_url": "https://img.youtube.com/vi/fJ9rUzIMcZQ/maxresdefault.jpg"
  }
]
```

#### Save Song
```
POST /api/save?force=true
Content-Type: multipart/form-data

# Form data:
file: [audio file binary]
```

#### Download from Spotify
```
POST /api/download
Content-Type: application/json

{
  "spotify_url": "https://open.spotify.com/track/4uLU6hMCjMI75M1A2tKUQC"
}
```

#### Clear Database
```
DELETE /api/erase
```

### API Examples

#### JavaScript/TypeScript

```javascript
// Identify a song
async function identifySong(audioFile) {
  const formData = new FormData();
  formData.append('file', audioFile);

  const response = await fetch('http://localhost:8080/api/find', {
   method: 'POST',
   body: formData
  });

  return response.json();
}

// Get library
async function getLibrary() {
  const response = await fetch('http://localhost:8080/api/library');
  return response.json();
}
```

#### Python
```python
import requests

def identify_song(file_path):
   with open(file_path, 'rb') as f:
      files = {'file': f}
      response = requests.post('http://localhost:8080/api/find', files=files)
   return response.json()

def get_library():
   response = requests.get('http://localhost:8080/api/library')
   return response.json()
```

#### Rust
```rust
use reqwest::multipart;

async fn identify_song(file_path: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
   let file = tokio::fs::File::open(file_path).await?;
   let file_part = multipart::Part::stream(file).file_name("audio.mp3");
   let form = multipart::Form::new().part("file", file_part);
   
   let client = reqwest::Client::new();
   let response = client
      .post("http://localhost:8080/api/find")
      .multipart(form)
      .send()
      .await?;
   
   Ok(response.json().await?)
}
```

For complete API documentation, visit the web interface at /api-docs or check out the API Documentation page.

## 🏗️ Project Architecture
![image](https://github.com/user-attachments/assets/e6f4cfbd-124b-40d7-9fc3-d9767b609e95)

### Directory Structure

```
acousti-scan-rs/
├── src/                          # Rust backend source code
│   ├── api.rs                   # REST API endpoints and server
│   ├── command_handlers.rs      # CLI command implementations
│   ├── db/                      # Database operations and models
│   ├── download/                # Spotify/YouTube integration
│   ├── models.rs                # Data structures and types
│   ├── shazam/                  # Audio fingerprinting algorithms
│   ├── utils/                   # Helper functions and utilities
│   ├── wav/                     # Audio processing and conversion
│   └── main.rs                  # Application entry point
├── client/                      # Next.js frontend application
│   ├── app/                     # Next.js 14 app router pages
│   ├── components/              # React components
│   ├── lib/                     # Utility functions and API client
│   └── styles/                  # CSS and styling files
├── songs/                       # Processed audio files storage
├── tmp/                         # Temporary file processing
├── Cargo.toml                   # Rust dependencies and metadata
└── README.md                    # This file
```

### Core Components

#### Backend Architecture
1. main.rs - CLI argument parsing and application entry point
2. api.rs - Actix Web server with CORS and multipart file handling
3. command_handlers.rs - Business logic for CLI operations
4. shazam/ - Core audio fingerprinting algorithms
5. db/ - SQLite database operations and schema
6. wav/ - Audio file processing and FFmpeg integration
7. download/ - External service integrations

#### Frontend Architecture
1. app/ - Next.js pages using the app router
2. components/ - Reusable React components
3. lib/api.ts - HTTP client for backend communication
4. contexts/ - React context providers

### Data Flow
Audio Input → FFmpeg Conversion → Spectrogram Analysis → Peak Detection → 
Fingerprint Generation → Database Storage/Matching → Results Display

<!-- Add data flow diagram -->
![Swimlane Diagram - Data Flow by Component](https://github.com/user-attachments/assets/e0d9eb79-72da-4836-88ae-b8c1728f1c7d)
![Sequence Diagram - Data Flow Between Components](https://github.com/user-attachments/assets/0952d1bc-24c9-4002-bd41-cb171f24f934)

## 🧪 Development

### Running Tests

```bash
# Run Rust backend tests
cargo test

# Run frontend tests
cd client
npm test
```

### Development Mode
```bash
# Backend with auto-reload
cargo install cargo-watch
cargo watch -x "run --release api-server"

# Frontend with hot reload
cd client
npm run dev
```

### Building for Production
```bash
# Build optimized Rust binary
cargo build --release

# Build optimized frontend
cd client
npm run build
npm start
```

### Environment Variables
Create .env files for configuration:

Backend (.env):
```
DATABASE_URL=./db.sqlite3
RUST_LOG=info
```

Frontend (client/.env.local):
```
NEXT_PUBLIC_API_URL=http://localhost:8080
```

### Database Schema
The SQLite database contains the following main tables:

- songs - Song metadata (title, artist, YouTube ID)
- fingerprints - Audio fingerprint hashes and timing data
- peaks - Spectral peaks for debugging and analysis
<!-- Add database schema diagram -->
![Database Schema](https://github.com/user-attachments/assets/60aff281-1c5f-4d0d-b8e5-163868c253b9)


## 🤝 Contributing
We welcome contributions! Please follow these steps:

1. Fork the repository
2. Create a feature branch
```bash
git checkout -b feature/amazing-feature
```

3. Make your changes
4. Add tests for new functionality
5. Commit your changes
```bash
git commit -m 'Add amazing feature'
```

6. Push to the branch
```bash
git push origin feature/amazing-feature
```

7. Open a Pull Request

### Development Guidelines
- Follow Rust conventions and use cargo fmt
- Write tests for new features
- Update documentation for API changes
- Use TypeScript for frontend contributions
- Follow the existing code style

### Issues and Feature Requests
- 🐛 Bug Reports: Use the bug report template
- 💡 Feature Requests: Use the feature request template
- 📚 Documentation: Help improve our docs
- 🎨 UI/UX: Enhance the user experience

## 📄 License
This project is licensed under the MIT License - see the LICENSE file for details.

```
MIT License

Copyright (c) 2024 Acousti-Scan RS Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
```

## 🎯 Roadmap

### Upcoming Features
- [ ] Multi-language Support - Internationalization
- [ ] Cloud Deployment - Docker containers and cloud setup
- [ ] Advanced Analytics - Detailed usage statistics and insights
- [ ] Playlist Management - Create and manage song collections
- [ ] Social Features - Share discoveries and collaborate

### Long-term Goals
- [ ] Machine Learning Integration - Improve identification accuracy
- [ ] Mobile Apps - Native iOS and Android applications
- [ ] Plugin System - Extensible architecture for custom analyzers
- [ ] Real-time Streaming - Live radio and streaming service integration
- [ ] Enterprise Features - Advanced deployment and management tools

## 📞 Contact & Support

### Project Maintainers
- Primary Maintainer: Ankesh Gupta
- GitHub: @Ankesh2004

### Community
- 🐛 Bug Reports: GitHub Issues
- 💬 Discussions: GitHub Discussions
- 📧 Email: support@acousti-scan.com
- 💬 Discord: Join our Discord Server

### Documentation
- 📖 API Docs: Available at /api-docs when running the application
- 🎓 Tutorials: Check out our Wiki
- 📚 Examples: See the examples/ directory

<div align="center">
Made with ❤️ and 🦀 Rust

[⭐ Star this repo](https://github.com/Ankesh2004/Acoustic-Scan) • [🐛 Report Bug](https://github.com/your-username/your-repo-name/issues/new?template=bug_report.md) • [💡 Request Feature](https://github.com/your-username/your-repo-name/issues/new?template=feature_request.md)
</div>
