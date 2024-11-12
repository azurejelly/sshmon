FROM rust:1.81 AS build-env

WORKDIR /build

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

WORKDIR /app

COPY --from=build-env /build/target/release/ssh-monitor /app

ENTRYPOINT [ "/app/ssh-monitor" ]
