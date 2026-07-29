ARG RUST_VERSION=1.95
FROM rust:${RUST_VERSION}-alpine AS build-backend
WORKDIR /src
# Various additional dependencies requried for compiling
RUN apk update && apk add openssl openssl-dev openssl-libs-static make g++ file 
RUN --mount=type=bind,source=src,target=src,readwrite \
    --mount=type=bind,source=libs,target=libs,readwrite \
    --mount=type=bind,source=rustdesk-api-server,target=rustdesk-api-server,readwrite \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock,readwrite \
    --mount=type=bind,source=db_v2.sqlite3,target=db_v2.sqlite3,readwrite \
    --mount=type=bind,source=build.rs,target=build.rs \
    --mount=type=cache,target=/src/target \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
# Database required for compilation time checking of SQL commands
DATABASE_URL=sqlite://db_v2.sqlite3 cargo build --release
# Copy out of cached directory - not available in second stage container otherwise...
cp target/release/hbbr target/release/hbbs target/release/rustdesk-utils /
EOF

FROM node:current-alpine AS build-frontend
WORKDIR /app
COPY ./rustdesk-api-server/rd-status/package.json ./rustdesk-api-server/rd-status/package-lock.json ./
RUN npm ci --no-fund --no-audit --ignore-scripts
COPY ./rustdesk-api-server/rd-status/ .
RUN npm run build --omit=dev --omit=optional --omit=peer

FROM scratch
USER 1001
COPY --from=build-backend /hbbr /hbbs /rustdesk-utils /usr/bin/
COPY --from=build-frontend --chmod=+rX /app/dist/rd-status/browser/ /var/www/html/frontend
WORKDIR /root
EXPOSE 21114 21115 21116 21116/udp 21118 21117 21119
