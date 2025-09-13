FROM rust:1.89-slim-trixie AS builder

RUN apt-get update \
    && apt-get install -y \
      cmake \
      pkg-config \
      libssl-dev \
      g++

WORKDIR /usr/src/giortes

COPY . .

RUN cargo install --path .

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y \
      ca-certificates \
      net-tools \
      libssl-dev \
      curl \
    && rm -rf /var/lib/apt/lists/* \
    && update-ca-certificates

COPY --from=builder /usr/local/cargo/bin/giortes /usr/local/bin/giortes

LABEL org.opencontainers.image.description="Name days"

ENV HOST        0.0.0.0
ENV PORT        8080
ENV RUST_LOG    giortes=info

EXPOSE          8080

CMD ["giortes"]
