#!/usr/bin/env bash

# Get process IDs for the HTTP (port 5000) and HTTPS (port 4443) servers.
HTTP_PID=$(sudo lsof -t -i:5000)
HTTPS_PID=$(sudo lsof -t -i:4443)

if [ -n "$HTTP_PID" ]; then
  sudo kill -9 $HTTP_PID
fi

if [ -n "$HTTPS_PID" ]; then
  sudo kill -9 $HTTPS_PID
fi
