FROM rust:1.81 AS build-env

ARG BUILD_TYPE=release

WORKDIR /build

COPY . .

RUN if [ "$BUILD_TYPE" = "release" ]; then cargo build --release; else cargo build; fi

FROM gcr.io/distroless/cc-debian12

WORKDIR /app

COPY --from=build-env /build/target/$(if [ "$BUILD_TYPE" = "release" ]; then echo "release"; else echo "debug"; fi)/ssh-monitor ./

ENTRYPOINT [ "ssh-monitor" ]
