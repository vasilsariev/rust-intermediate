FROM rust:slim-buster as builder

RUN cargo new --bin rust-docker
WORKDIR ./rust-docker
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /rust-docker/target/release/rust-docker ./rust-docker
CMD ["./rust-docker"]