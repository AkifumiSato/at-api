# release
FROM rust:1.44.1
WORKDIR /migration_tmp
COPY . .
RUN cargo install diesel_cli
CMD ["diesel setup"]
