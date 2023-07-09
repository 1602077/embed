# grpc embedding server
#
# build uses:
#   * protoc to generate protobuf type definitions.
#   * cargo-chef to cache build deps for a faster build time.
#   * debian:trixie-slim runtime pinned by sha256.
#
# args:
#   * RUST_BINARY: binary to build for [server/client].
#
FROM rust:1.70 as base
RUN apt-get update
RUN apt-get --assume-yes install protobuf-compiler
RUN cargo install cargo-chef
WORKDIR /app

# ----------------------------------------------------------------------------

FROM base AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ----------------------------------------------------------------------------

FROM base AS builder 
ARG RUST_BINARY="server"
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build \
    --bin "${RUST_BINARY}" \
    --release

# ----------------------------------------------------------------------------

# FIXME: This should ideally be running in a scratch container to remove the 
# shell, but as just a personal project I do not want to go through the 
# effort of a cross-compilation with MUSL to support this.

FROM debian@sha256:d8151683a429d6582b688ed9c5e10a2c40a8100427c54d5b6c69a0aa40c0b3c1
ARG RUST_BINARY="server"
WORKDIR /app
COPY --from=builder /app/target/release/${RUST_BINARY} /app/entrypoint
# TODO: Switch to config being mounted as a k8s volume once server application 
# written.
COPY --from=builder /app/config /app/config
EXPOSE 50051
ENV APPLICATION__RUNTIME_ENVIRONMENT=prod
CMD ["./entrypoint"]
