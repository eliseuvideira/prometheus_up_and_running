FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM debian:latest AS runner

RUN apt-get update && apt-get install -y tini

COPY --from=builder /usr/src/app/target/release/http_service /usr/src/app/http_service

CMD ["/usr/bin/tini", "--", "/usr/src/app/http_service"]
