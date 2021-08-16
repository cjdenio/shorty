FROM rust:1.54

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM debian:buster

WORKDIR /usr/src/app

RUN apt-get update -y && apt-get install -y libpq5

COPY --from=0 /usr/src/app/target/release/shorty ./app
COPY --from=0 /usr/src/app/templates ./templates

CMD ["./app"]
