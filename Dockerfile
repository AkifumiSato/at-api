# build-stage
FROM rust:1.44.1 AS production
WORKDIR /app
COPY . .
RUN cargo build --release
RUN cargo install diesel_cli
EXPOSE 8088
CMD ["diesel setup", "&&", "/api/my_app"]

# database
FROM postgres:11-alpine AS db
ENV LANG ja_JP.utf8

# dev
FROM rust:1.44.1 AS develop
WORKDIR /app
RUN cargo install cargo-watch
RUN cargo install diesel_cli
COPY . .
