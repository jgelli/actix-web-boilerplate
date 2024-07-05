FROM rust:latest

RUN cargo install cargo-watch

WORKDIR /api

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY . .

EXPOSE 8081

CMD ["cargo", "watch", "-x", "run"]
