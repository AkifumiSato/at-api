# build-stage
FROM rust:1.44.1 AS build-stage

RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

RUN USER=root cargo new my_app
WORKDIR /app/my_app

COPY Cargo.toml Cargo.lock ./
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/my_app*

COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN cargo install diesel_cli

# production
FROM alpine:latest AS production
COPY --from=build-stage /app/my_app/target/x86_64-unknown-linux-musl/release/my_app /usr/local/bin/my_app
CMD ["my_app"]


# database
FROM postgres:11-alpine AS db
ENV LANG ja_JP.utf8

# dev
FROM rust:1.44.1 AS develop
WORKDIR /app
RUN cargo install cargo-watch
RUN cargo install diesel_cli
COPY . .
