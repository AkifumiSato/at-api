# database
FROM postgres:11-alpine as db
ENV LANG ja_JP.utf8

# dev
FROM rust:1.44.1 as develop
WORKDIR /app
RUN cargo install cargo-watch
RUN cargo install diesel_cli
COPY . .

# build-stage
FROM rust:1.44.1-slim-stretch as build-stage
RUN cargo build --release

# release-stage
FROM build-stage as release-stage
RUN cargo install diesel_cli
RUN diesel setup

# production
FROM scratch as production
COPY --from=build-stage /app/target/release/my_app /api
EXPOSE 8088
CMD ["diesel setup", "&&", "/api/my_app"]
