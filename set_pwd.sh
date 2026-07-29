#!/bin/bash
docker run --rm -it --volume ./data:/var/db/:z --user 1001:1001 registry.gitlab.com/msv-sw/rustdesk-api-server/main-pwd:latest