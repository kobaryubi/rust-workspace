FROM rust:1.75
WORKDIR /usr/src/rust-workspace
RUN rustup component add rustfmt