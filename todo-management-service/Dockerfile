FROM rust:1.81 as builder
LABEL authors="Revanth Shalon Raj"
WORKDIR /usr/src/todo-management-service
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/todo-management-service/target/release/todo-management-service /usr/local/bin/todo-management-service
CMD ["todo-management-service"]