FROM rust:1.67 as builder
WORKDIR /usr/src/market
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN rm -rf /var/lib/apt/lists/*

EXPOSE 8080

# apt-get update && apt-get install -y extra-runtime-dependencies && 
COPY --from=builder /usr/local/cargo/bin/market /usr/local/bin/market
CMD ["market"]