FROM rust:1.49 as builder

RUN USER=root cargo new --bin bacdb
WORKDIR ./bacdb
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD ./ .

RUN rm ./target/release/deps/backend*
RUN cargo build --release

FROM debian:buster-slim

EXPOSE 8080

COPY --from=builder /bacdb/target/release/backend .
CMD ["./backend"]

