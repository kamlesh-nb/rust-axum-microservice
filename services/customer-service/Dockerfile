FROM rust:latest as build

WORKDIR /customer
COPY ./common ./common
COPY ./customer-service ./customer-service

WORKDIR /customer/customer-service

RUN cargo build --release

EXPOSE 8080

FROM debian:buster-slim
WORKDIR /customer

RUN apt-get update
RUN apt-get -y install libssl-dev

RUN cd /lib/x86_64-linux-gnu
RUN ln -s libssl.so.1.1.0 libssl.so.1.1
RUN ln -s libcrypto.so.1.1.0 libcrypto.so.1.1

COPY --from=build /customer/customer-service/appconfig.Dev.toml .
COPY --from=build /customer/customer-service/target/release/customer-service .

CMD ["./customer-service"]