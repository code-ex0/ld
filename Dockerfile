# build image for the blockchain

FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:buster-slim as runtime
COPY --from=builder /usr/local/cargo/bin/ld /usr/local/bin/ld
EXPOSE 8000
CMD ["ld"]
