FROM rust:alpine as builder
RUN apk update
RUN apk add --no-cache musl-dev pkgconf
WORKDIR /service
COPY Cargo.* .
COPY src ./src
RUN cargo build --release

FROM scratch as runtime
WORKDIR /service
COPY --from=builder /service/target/release/dummy-service .
COPY --from=builder /service/target/release/consumer-service .
CMD ["./dummy-service"]
