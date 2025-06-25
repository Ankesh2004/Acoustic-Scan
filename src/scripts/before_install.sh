#!/usr/bin/env bash

# Update package lists
sudo apt-get -y update

# Install Rust toolchain via rustup if not already installed
if ! command -v rustc >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Install NodeJS and npm (for client-side build)
sudo apt-get -y install nodejs npm

# Install ffmpeg (for audio processing)
sudo apt-get -y install ffmpeg

# Install Certbot for SSL certificates
DOMAIN="localport.online"
EMAIL="hemlock@gmail.com"
CERT_DIR="/etc/letsencrypt/live/$DOMAIN"

if [ ! -d "$CERT_DIR" ]; then
    sudo apt-get install -y certbot
    sudo certbot certonly --standalone -d $DOMAIN --email $EMAIL --agree-tos --non-interactive
    if [ $? -eq 0 ]; then
        sudo apt-get -y install acl
        sudo setfacl -m u:ubuntu:--x /etc/letsencrypt/archive
    fi
fi

# Install MongoDB only if not already installed
if [ ! -f "/usr/bin/mongod" ]; then
    sudo apt-get install -y gnupg curl
    curl -fsSL https://www.mongodb.org/static/pgp/server-7.0.asc | \
       sudo gpg -o /usr/share/keyrings/mongodb-server-7.0.gpg --dearmor
    echo "deb [ arch=amd64,arm64 signed-by=/usr/share/keyrings/mongodb-server-7.0.gpg ] https://repo.mongodb.org/apt/ubuntu jammy/mongodb-org/7.0 multiverse" | sudo tee /etc/apt/sources.list.d/mongodb-org-7.0.list
    sudo apt-get update
    sudo apt-get install -y mongodb-org mongosh
fi

# Remove any previous project folder to start fresh
sudo rm -rf /home/ubuntu/song-recognition
