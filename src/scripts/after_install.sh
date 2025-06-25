#!/usr/bin/env bash

# Change ownership of the project folder (adjust user/group as needed)
sudo chown -R ubuntu:ubuntu /home/ubuntu/song-recognition

# Start and enable the MongoDB service (if using MongoDB)
sudo systemctl start mongod.service
sudo systemctl enable mongod.service
