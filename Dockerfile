FROM rust:1-buster as builder
WORKDIR /usr/src/amethyst
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/amethyst /usr/local/bin/amethyst
CMD ["amethyst"]
