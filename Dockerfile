LABEL authors="pAkalpa"

FROM rust:1.76-alphine as builder

WORKDIR /app

COPY . .

RUN cargo install --path .

FROM alpine:3.19.1 as runner

COPY --from=builder /usr/local/cargo/bin/dynamic_update_server /usr/local/bin/dynamic_update_server

ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8000

ENTRYPOINT ["dynamic_update_server"]