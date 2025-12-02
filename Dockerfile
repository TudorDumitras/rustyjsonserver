# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.91.1

FROM rust:${RUST_VERSION}-alpine AS build
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git

# Build the app
RUN --mount=type=bind,source=./src,target=/app/src \
    --mount=type=bind,source=./Cargo.toml,target=/app/Cargo.toml \
    --mount=type=bind,source=./Cargo.lock,target=/app/Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release --bin rustyjsonserver && \
    cp ./target/release/rustyjsonserver /bin/server

# Create a new stage for running the application.

FROM alpine:3.18 AS final

# Create the directory for the application.
RUN mkdir -p /app

# Copy the executable and configuration file from the "build" stage.
COPY --from=build /bin/server /bin/
COPY ./config.json /app/config.json

# Expose the port that the application listens on.
EXPOSE 8080

# test connectivity with netcat: install and listen on port 5555
# RUN apk add --no-cache netcat-openbsd
# CMD ["nc", "-l", "5555"] 

# run: rjserver serve --config config.json
ENTRYPOINT ["/bin/server"]
CMD ["serve", "--config", "/app/config.json"]
