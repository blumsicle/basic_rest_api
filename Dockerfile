FROM rust:bullseye as build

WORKDIR /build
RUN cargo new basic_rest_api --bin
WORKDIR /build/basic_rest_api
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
COPY src src/
RUN touch src/main.rs
RUN cargo build --release

FROM debian:bullseye

ENV SERVER.HOST=0.0.0.0
ENV SERVER.PORT=8080

WORKDIR /app
COPY --from=build /build/basic_rest_api/target/release/basic_rest_api .
CMD ["./basic_rest_api"]

EXPOSE 8080
