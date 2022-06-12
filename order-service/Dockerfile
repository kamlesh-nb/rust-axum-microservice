FROM rust:latest as build

WORKDIR /usr/src/order-service
COPY . .

RUN cargo build --release

EXPOSE 8080

FROM debian:buster-slim
COPY --from=build /usr/src/order-service/target/release/order-service /usr/local/bin/order-service
ENTRYPOINT ["/usr/local/bin/order-service"]