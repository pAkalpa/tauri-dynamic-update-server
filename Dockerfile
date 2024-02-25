FROM rust:1.76 as builder

WORKDIR /app

COPY . .

RUN cargo install --path .

FROM debian:buster-slim as runner

COPY --from=builder /usr/local/cargo/bin/dynamic_update_server /usr/local/bin/dynamic_update_server

ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8000

RUN apt-get update && apt install -y openssl && rm -rf /var/cache/apt/archives /var/lib/apt/lists/*

ENTRYPOINT ["dynamic_update_server"]