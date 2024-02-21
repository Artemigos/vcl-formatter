FROM rust:1.75-slim as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build-env /app/target/release/vcl-formatter /
ENTRYPOINT ["./vcl-formatter"]
