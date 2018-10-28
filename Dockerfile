FROM rust:latest

WORKDIR /usr/src/rust_postgres
COPY . .

RUN cargo install --bin service
