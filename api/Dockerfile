FROM rustlang/rust:nightly

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo install cargo-watch

ENV ROCKET_ADDRESS=0.0.0.0

ENV ROCKET_PORT=8000

WORKDIR /usr/src/app

EXPOSE 8000

VOLUME ["/usr/local/cargo"]
