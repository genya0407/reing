FROM ubuntu AS builder

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN apt-get update
RUN apt-get install -y curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rustup.sh
RUN sh /tmp/rustup.sh -y --no-modify-path --default-toolchain nightly
RUN chmod -R a+w $RUSTUP_HOME $CARGO_HOME;
RUN rm /tmp/rustup.sh

WORKDIR /app
ADD rust-toolchain .
RUN rustup toolchain install $(cat rust-toolchain)
RUN rustup override set $(cat rust-toolchain)

# tzdataのインストール時にインタラクティブモードになるのを防ぐ
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get install -y build-essential libssl-dev pkg-config libpq-dev
ENV TZ=Asia/Tokyo
RUN mkdir src
RUN echo 'fn main() { println!("Hello, world!"); }' > src/main.rs
ADD Cargo.toml .
ADD Cargo.lock .
RUN cargo build --release
RUN rm src/main.rs && rm ./target/release/deps/reing*
RUN cargo install diesel_cli --no-default-features --features "postgres"
ADD . .
RUN cargo build --release

FROM ubuntu
WORKDIR /app
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update
RUN apt-get install -y build-essential libssl-dev pkg-config libpq-dev
COPY . .
COPY --from=builder /app/target/release/reing /usr/bin/reing
