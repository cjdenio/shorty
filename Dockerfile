FROM rust:1.50

WORKDIR /usr/src/app

COPY . .

RUN rustup toolchain install nightly && \
    rustup default nightly

RUN cargo build --release

FROM debian:buster

COPY --from=0 /usr/src/app/target/release/shorty /usr/bin/app

CMD ["app"]