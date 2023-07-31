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
# FROM clux/muslrust:stable as base
#FROM rust:bookworm as base
#RUN apt-get update -y && \
#    apt-get install -y protobuf-compiler build-essential
#WORKDIR /app
#ARG RUST_BINARY="server"
#COPY . .
#RUN cargo build \
#    --bin "${RUST_BINARY}" \
#    --release

## ----------------------------------------------------------------------------

## FIXME: This should ideally be running in a scratch container to remove the 
## shell, but as just a personal project I do not want to go through the 
## effort of a cross-compilation with MUSL to support this.

#FROM debian
## FROM debian@sha256:d8151683a429d6582b688ed9c5e10a2c40a8100427c54d5b6c69a0aa40c0b3c1
#ARG RUST_BINARY="server"
#WORKDIR /app
#COPY --from=builder /app/target/release/${RUST_BINARY} /app/entrypoint
## TODO: Switch to config being mounted as a k8s volume once server application 
## written.
#COPY --from=builder /app/config /app/config
#EXPOSE 50051
#ENV APPLICATION__RUNTIME_ENVIRONMENT=prod
#CMD ["./entrypoint"]


FROM rust:bookworm as builder

ARG OPENSSL_VERSION=1.1.1m

RUN apt-get update && \
    export DEBIAN_FRONTEND=noninteractive && \
    apt-get install -yq \
        build-essential \
        cmake \
        curl \
        file \
        git \
        graphviz \
        musl-dev \
        musl-tools \
        libpq-dev \
        libssl-dev \
        linux-libc-dev \
        protobuf-compiler \
        pkgconf \
        sudo \
        unzip \
        xutils-dev \
        && \
    apt-get clean && rm -rf /var/lib/apt/lists/* && \
    useradd rust --user-group --create-home --shell /bin/bash --groups sudo

RUN ln -s "/usr/bin/g++" "/usr/bin/musl-g++"

RUN echo "Building OpenSSL" && \
    ls /usr/include/linux && \
    mkdir -p /usr/local/musl/include && \
    ln -s /usr/include/linux /usr/local/musl/include/linux && \
    ln -s /usr/include/x86_64-linux-gnu/asm /usr/local/musl/include/asm && \
    ln -s /usr/include/asm-generic /usr/local/musl/include/asm-generic && \
    cd /tmp && \
    short_version="$(echo "$OPENSSL_VERSION" | sed s'/[a-z]$//' )" && \
    curl -fLO "https://www.openssl.org/source/openssl-$OPENSSL_VERSION.tar.gz" || \
        curl -fLO "https://www.openssl.org/source/old/$short_version/openssl-$OPENSSL_VERSION.tar.gz" && \
    tar xvzf "openssl-$OPENSSL_VERSION.tar.gz" && cd "/tmp/openssl-$OPENSSL_VERSION" 

WORKDIR /tmp/openssl-${OPENSSL_VERSION}
RUN env CC=musl-gcc ./config --prefix=/usr/local.musl

# RUN env CC=musl-gcc "/tmp/openssl-${OPENSSL_VERSION}/config" no-shared no-zlib -fPIC --prefix=/usr/local/musl -DOPENSSL_NO_SECURE_MEMORY linux-x86_64
# RUN env C_INCLUDE_PATH=/usr/local/musl/include/ make depend

RUN env C_INCLUDE_PATH=/usr/local/musl/include/ make

RUN make install && \
    rm /usr/local/musl/include/linux /usr/local/musl/include/asm /usr/local/musl/include/asm-generic && \
    rm -r /tmp/*


# RUN rustup target add x86_64-unknown-linux-musl

# WORKDIR /app
# RUN apt install openssl
# COPY . .
# ENV \
#   CC_x86_64_unknown_linux_musl=clang \
#   RUST_BACKTRACE=full

# RUN cargo build --target x86_64-unknown-linux-musl --release --bin server
# # RUN \
# #   --mount=type=cache,target=/app/target,rw \
# #   --mount=type=cache,target=/usr/local/cargo/registry,rw \
# #   cargo build --target x86_64-unknown-linux-musl --release --bin server

# # ----------------------------------------------------------------------------

# FROM alpine:3.17.3
# WORKDIR /app
# COPY --from=builder /app/config /app/config
# COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/server /app/entrypoint
# ENV APPLICATION__RUNTIME_ENVIRONMENT=prod
# EXPOSE 50051
# CMD ["./entrypoint"]

