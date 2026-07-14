#!/bin/bash
# Update source
git pull

# Get communication library
git submodule update --recursive --remote

# Stop containers
docker compose down

# Build image
docker compose build

# Prepare data directory (key); only required on first run
mkdir -p data
chmod 750 data

# Start containers in detached mode
docker compose up --remove-orphans -d

# Cleanup
docker image prune -a -f
