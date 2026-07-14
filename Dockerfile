ARG RUST_VERSION=1.95
FROM rust:${RUST_VERSION}-alpine AS build
WORKDIR /src
# Various additional dependencies requried for compiling
RUN apk update && apk add openssl openssl-dev openssl-libs-static make g++ file 
RUN --mount=type=bind,source=build/src,target=src,readwrite \
    --mount=type=bind,source=build/libs,target=libs,readwrite \
    --mount=type=bind,source=build/Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=build/Cargo.lock,target=Cargo.lock,readwrite \
    --mount=type=bind,source=build/db_v2.sqlite3,target=db_v2.sqlite3,readwrite \
    --mount=type=bind,source=build/build.rs,target=build.rs \
    --mount=type=cache,target=/src/target \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
# Database required for compilation time checking of SQL commands
DATABASE_URL=sqlite://db_v2.sqlite3 cargo build --release
# Copy out of cached directory - not available in second stage container otherwise...
cp target/release/hbbr target/release/hbbs target/release/rustdesk-utils /
EOF

FROM scratch
COPY --from=build /hbbr /hbbs /rustdesk-utils /usr/bin/
WORKDIR /root
EXPOSE 21115 21116 21116/udp 21118 21117 21119
