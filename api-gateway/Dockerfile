FROM rust:1.81 as builder
LABEL authors="Revanth Shalon Raj"
WORKDIR /usr/src/api-gateway
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/api-gateway/target/release/api-gateway /usr/local/bin/api-gateway
CMD ["api-gateway"]