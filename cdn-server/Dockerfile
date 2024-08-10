FROM rust:latest

WORKDIR /usr/src/app

COPY . .
COPY ../Cargo.toml ./Cargo.toml
COPY ../application.toml ./application.toml

RUN cargo build --release
CMD ["cargo", "run", "--release", "--bin", "cdn-server"]