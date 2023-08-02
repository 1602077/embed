# grpc embedding server
#
# build uses:
#   * protoc to generate protobuf type definitions.
#   * build-essential to build libtorch (pytorch).
#
# args:
#   * RUST_BINARY: binary to build for [server/client].
# 
# future work (a.k.a not worth it for a personal project):
#   * prod image should be a scratch / distroless image to remove the shell.
#   * config should be mounted as a k8s volume instead of being baked into image.
#   * musl cross compilation so amd images can be built on arm architectures.
#

FROM rust:bookworm as builder
RUN apt-get update -y && \
    apt-get install -y protobuf-compiler build-essential cmake clang

# install libtorch
RUN wget https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-2.0.1%2Bcpu.zip -O libtorch.zip && \
 unzip -o libtorch.zip
ENV LIBTORCH=/libtorch \
  LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH

WORKDIR /app
ARG RUST_BINARY="server"
COPY . .
RUN cargo build \
    --bin "${RUST_BINARY}" \
    --release

# ----------------------------------------------------------------------------

FROM debian:bookworm-slim
ARG RUST_BINARY="server"
ARG RUSTBERT_CACHE="/app/.cache"
RUN mkdir -p $RUSTBERT_CACHE
COPY --from=builder /libtorch /libtorch

WORKDIR /app
COPY --from=builder /app/target/release/${RUST_BINARY} /app/entrypoint
COPY --from=builder /app/config /app/config
EXPOSE 50051
ENV APPLICATION__RUNTIME_ENVIRONMENT=prod \
  RUSTBERT_CACHE=${RUSTBERT_CACHE} \
  LIBTORCH=/libtorch \
  LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
CMD ["./entrypoint"]

