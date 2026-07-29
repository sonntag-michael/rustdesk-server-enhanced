#!/bin/bash

if [ ! -f .env ]
then
    echo "Please create .env file first (based on .env.example)!"
    exit
fi

# Update source
git pull

# Get communication library and API server
git submodule update --init --recursive --remote

# Stop containers
docker compose down

# Build image and start containers in detached mode
#docker compose -f docker-compose.yml -f docker-compose.local.yml up --build --remove-orphans -d
# Retrieve image from repository and start containers in detached mode
docker compose up --build --remove-orphans -d

# Cleanup
docker image prune -a -f
