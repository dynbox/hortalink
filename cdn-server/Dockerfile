# Build Stage
FROM rust:1.77 AS builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new cdn-server
WORKDIR /usr/src/cdn-server
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src

# Bundle Stage
FROM scratch
COPY --from=builder /usr/src/cdn-server/target/x86_64-unknown-linux-musl/release/cdn-server .
CMD ["./cdn-server"]