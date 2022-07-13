FROM rust:1.61 as builder
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
EXPOSE 8080
COPY --from=builder ./target/release/apigelo ./target/release/apigelo
CMD ["/target/release/apigelo"]