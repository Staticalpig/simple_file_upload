# ---- Build stage ----
# Edition 2024 requires Rust 1.85+, so track the latest stable image.
FROM rust:1-bookworm AS builder
WORKDIR /app

# Cache dependencies: build with stub sources first so this layer is only
# invalidated when Cargo.toml / Cargo.lock change.
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && touch src/lib.rs \
    && cargo build --release \
    && rm -rf src

# Build the real application.
COPY src ./src
RUN touch src/main.rs src/lib.rs && cargo build --release

# ---- Runtime stage ----
FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --no-create-home appuser

COPY --from=builder /app/target/release/file_upload /usr/local/bin/file_upload

USER appuser
EXPOSE 8080
CMD ["file_upload"]