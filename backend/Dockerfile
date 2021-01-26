FROM rust:1.49 as builder
MAINTAINER Matei Tinca (tinca.matei02@gmail.com)

RUN USER=root cargo new --bin bacdb
WORKDIR ./bacdb
COPY ./Cargo.toml ./Cargo.toml
RUN printf >src/main.rs "extern crate diesel; \
#[macro_use] \
extern crate diesel_migrations; \
extern crate dotenv; \
fn main() { \
    println!(\"Hello world!\"); \
}"

RUN cargo build --release
RUN rm src/*.rs

ADD ./ .

RUN rm target/release/deps/backend*
RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/bacdb

EXPOSE 8080
RUN apt-get update && apt-get -y install libpq5 postgresql && rm -rf /var/lib/apt/lists/*
COPY --from=builder /bacdb/target/release/backend .
COPY --from=builder /bacdb/.env .
COPY wait-for-postgres.sh startup.sh
RUN chmod +x startup.sh
ENTRYPOINT ["./startup.sh"]

