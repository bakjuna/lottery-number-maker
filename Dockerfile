FROM rust:1.73.0 AS chef

WORKDIR /usr/src/lottery-number-maker

RUN cargo install cargo-chef; \
    rm -rf $CARGO_HOME/registry

FROM chef as planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /usr/src/lottery-number-maker/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json
RUN apt install pkg-config

COPY . .
RUN cargo build --release

FROM ubuntu:22.04

WORKDIR /app

COPY --from=builder /usr/src/lottery-number-maker/target/release/lottery-number-maker .

CMD ["./lottery-number-maker"]