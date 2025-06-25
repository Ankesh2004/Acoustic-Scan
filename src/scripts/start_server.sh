#!/usr/bin/env bash

start_server() {
    cd /home/ubuntu/song-recognition

    # Set environment variables for HTTPS server
    export SERVE_HTTPS="true"
    export CERT_KEY="/etc/letsencrypt/live/localport.online/privkey.pem"
    export CERT_FILE="/etc/letsencrypt/live/localport.online/fullchain.pem"

    # Build the Rust project in release mode
    cargo build --release

    # Optionally, grant the binary permission to bind to privileged ports (if needed)
    sudo setcap CAP_NET_BIND_SERVICE+ep target/release/song_recognition

    # Start the server in background, redirecting output to a log file
    nohup target/release/song_recognition serve -proto https -p 4443 > backend.log 2>&1 &
}

start_client() {
    cd /home/ubuntu/song-recognition/client
    npm install
    npm run build
    nohup npx serve -s build > client.log 2>&1 &
}

start_server
