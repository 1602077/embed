SHELL=/bin/bash
.DEFAULT_GOAL=help

.PHONY: build
build: # builds docker image.
	@export LIBTORCH=$(brew --cellar pytorch)/$(brew info --json pytorch | jq -r '.[0].installed[0].version')
	@export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
	docker build --build-arg="RUST_BINARY=server" -t embed-server .

.PHONY: server
server: # run grpc server locally.
	@cargo run --bin server

.PHONY: help
help: # shows help message.
	@egrep -h '\s#\s' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?# "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
