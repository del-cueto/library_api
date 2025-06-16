# Stage 1: Build the Rust application
FROM rust:latest AS builder
LABEL authors="delcueto"

# Install dependencies for SQLite compilation
RUN apt-get update && apt-get install -y libsqlite3-dev pkg-config ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/library_api

# Copy manifest files first to cache dependencies
COPY Cargo.toml Cargo.lock ./
# Remove lockfile if incompatible so container can regenerate it
RUN rm Cargo.lock || true
# Copy source code and migrations
COPY src ./src
COPY migrations ./migrations

# Generate a new lockfile and build in release mode
RUN cargo generate-lockfile
RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libsqlite3-0 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/library_api/target/release/library_api /usr/local/bin/library_api
COPY --from=builder /usr/src/library_api/migrations ./migrations

ENV DATABASE_URL=sqlite://./library.db
ENV JWT_SECRET=default-secret

EXPOSE 3000
CMD ["library_api"]

