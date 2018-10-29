# rust build image
FROM rust:latest as build

# shell
RUN USER=root cargo new --lib rust_postgres_service
WORKDIR /rust_postgres_service

# project deps
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# cache deps, remove shell src
RUN cargo build --release
RUN rm -r src/*

# copy project src tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/*
RUN cargo build --bin service --release

# final base - for smaller image size
FROM debian:stretch-slim

RUN apt-get update
RUN apt-get install --assume-yes wget ca-certificates gnupg lsb-release
RUN wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add -

RUN sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt/ $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'

RUN apt-get update
RUN apt-get upgrade --assume-yes
RUN apt-get install --assume-yes libpq-dev

# copy the build artifact from the build stage
COPY --from=build /rust_postgres_service/target/release/service .

EXPOSE 5000

# run the web server
CMD ./service
