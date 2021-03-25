FROM rust:1.50

WORKDIR /usr/src/app

COPY . .

RUN rustup toolchain install nightly && \
    rustup default nightly

RUN cargo build --release

FROM debian:buster

WORKDIR /usr/src/app

RUN apt-get install libpq5

COPY --from=0 /usr/src/app/target/release/shorty ./app

CMD ["./app"]
