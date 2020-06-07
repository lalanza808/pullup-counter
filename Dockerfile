FROM ubuntu:18.04
WORKDIR /srv
COPY . .
RUN apt-get update && apt-get install curl -y
RUN apt-get update && apt-get install build-essential -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN $HOME/.cargo/bin/rustup override set nightly
RUN $HOME/.cargo/bin/cargo build --release
CMD ./target/release/pullup-counter
