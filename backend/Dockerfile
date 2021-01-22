FROM rust:1.49 as builder
MAINTAINER Matei Tinca (tinca.matei02@gmail.com)

RUN USER=root cargo new --bin bacdb
WORKDIR ./bacdb
COPY ./Cargo.toml ./Cargo.toml
RUN cargo run --release
RUN rm src/*.rs

ADD ./ .

RUN rm target/release/deps/backend*
RUN cargo build --release

FROM debian:buster-slim

EXPOSE 8080
RUN apt-get update && apt-get -y install libpq5 postgresql && rm -rf /var/lib/apt/lists/*
COPY --from=builder /bacdb/target/release/backend .
COPY --from=builder /bacdb/.env .
CMD ["./backend"]

