FROM rust:alpine3.17

RUN apk add --update \
    build-base \
    pkgconfig

WORKDIR /app

COPY Cargo.lock /app/Cargo.lock
COPY Cargo.toml /app/Cargo.toml
COPY src /app/src

#RUN cargo vendor --no-delete
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/app/target <<EOF
    set -e
    touch /app/src/main.rs
    cargo build
EOF

RUN --mount=type=cache,target=/app/target cp target/debug/iron_mc .

CMD ./iron_mc