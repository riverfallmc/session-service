FROM rust:latest AS builder

WORKDIR /root/build

COPY . .

RUN cargo build --release

# Runtime
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
  apt-get install -y libssl3 libpq5 ca-certificates && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

COPY --from=builder /root/build/target/release/session-service ./

ENTRYPOINT ["/app/session-service"]