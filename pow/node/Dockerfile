FROM rust:alpine as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM rust:alpine as service
RUN apk update && apk upgrade && apk add openssl
COPY --from=builder /usr/loacl/cargo/bin/app /usr/local/bin/app
CMD ["app"]