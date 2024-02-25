FROM rust:bookworm as builder

WORKDIR /app

COPY . .

RUN cargo install --path .

FROM debian:bookworm-slim as runner

RUN apt-get update && apt install -y openssl

COPY --from=builder /usr/local/cargo/bin/dynamic_update_server /usr/local/bin/dynamic_update_server

ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8000

ENTRYPOINT ["dynamic_update_server"]