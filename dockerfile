FROM rust:1.87.0-slim as builder

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/config config
COPY --from=builder /usr/src/target/release/loco_benchmark-cli loco_benchmark-cli

ENTRYPOINT ["/usr/app/loco_benchmark-cli"]
