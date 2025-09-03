FROM rust:latest

WORKDIR /zkp-server

COPY . .

RUN apt-get update
RUN apt install -y protobuf-compiler

RUN cargo build --release --bin server --bin client
