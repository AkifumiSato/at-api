# build-stage
FROM rust:1.44.1 AS build-stage

WORKDIR /app

RUN USER=root cargo new at-api
WORKDIR /app/at-api

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
COPY . .
RUN rm ./target/release/deps/at_api*
RUN cargo build --release
RUN cargo install diesel_cli

# production
FROM gcr.io/distroless/cc-debian10 AS production
COPY --from=build-stage /app/at-api/target/release/at-api .
CMD ["./at-api"]

# database
FROM postgres:11-alpine AS db
ENV LANG ja_JP.utf8

# dev
FROM rust:1.44.1 AS develop
WORKDIR /app
RUN cargo install cargo-watch
RUN cargo install diesel_cli
COPY . .
