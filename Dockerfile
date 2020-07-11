# dev
FROM rust:1.44.1 as develop-stage
WORKDIR /app
RUN cargo install cargo-watch
COPY . .

# build
FROM develop-stage as build-stage
RUN cargo build --release

# production
FROM rust:1.44.1-slim-stretch
COPY --from=build-stage /app/target/release/api .
EXPOSE 8088
CMD ["/usr/local/bin/api"]
