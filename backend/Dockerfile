FROM rust:bullseye as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bullseye
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/app
EXPOSE 8000

ENV ROCKET_PORT=8000
ENV ROCKET_ADDRESS=0.0.0.0
ENV DB_USER=""
ENV DB_PASSWORD=""
ENV DB_URI=""

CMD ["app"]
