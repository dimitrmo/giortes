FROM rust:1.60.0-bullseye as builder
RUN apt update && apt install cmake -y
WORKDIR /usr/src/giortes
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && \
    apt-get install -y \
        curl \
        ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    update-ca-certificates
COPY --from=builder /usr/local/cargo/bin/giortes /usr/local/bin/giortes

LABEL org.opencontainers.image.description="Name days"

ENV TIMEOUT     600000
ENV HOST        0.0.0.0
ENV PORT        8080
ENV RUST_LOG    giortes=info

EXPOSE          8080

CMD ["giortes"]
