# grpc embedding server
#
# build uses:
#   * protoc to generate protobuf type definitions.
#   * cargo-chef to cache build deps for a faster build time.
#   * debian:trixie-slim runtime pinned by sha256.
#
# args:
#   * RUST_BINARY: binary to build for [server/client].
#   * RUST_TARGET: 
#
FROM rust:bookworm as base
RUN apt-get update -y && \
    apt-get install -y protobuf-compiler build-essential
WORKDIR /app
ARG RUST_BINARY="server"
COPY . .
RUN cargo build \
    --bin "${RUST_BINARY}" \
    --release

# ----------------------------------------------------------------------------

# FIXME: This should ideally be running in a scratch container to remove the 
# shell, but as just a personal project I do not want to go through the 
# effort of a cross-compilation with MUSL to support this.

FROM debian
ARG RUST_BINARY="server"
WORKDIR /app
COPY --from=builder /app/target/release/${RUST_BINARY} /app/entrypoint
# TODO: Switch to config being mounted as a k8s volume once server application 
# written.
COPY --from=builder /app/config /app/config
EXPOSE 50051
ENV APPLICATION__RUNTIME_ENVIRONMENT=prod
CMD ["./entrypoint"]

