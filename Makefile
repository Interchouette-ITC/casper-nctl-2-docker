# Define the current directory and Docker Compose command
CURRENT_DIR = .
DEV_DC = docker compose -f $(CURRENT_DIR)/docker-compose.yml

# Extract the second argument from MAKECMDGOALS, defaulting to "stable" if not specified
PROFILE ?= $(word 2,$(MAKECMDGOALS))
PROFILE := $(if $(PROFILE),$(PROFILE),stable)

# Define the image name
IMAGE_NAME=interchouette/casper-nctl-2-docker

# Build the Docker image for the specified profile (default is stable)
build:
	$(DEV_DC) --profile $(PROFILE) build

# Build the Docker image without using cache (for a fresh build)
build-no-cache:
	$(DEV_DC) --profile $(PROFILE) build --no-cache

# Build the Docker image and start the container with logs
build-start-log: build-no-cache
	$(DEV_DC) --profile $(PROFILE) up --remove-orphans

# Build the Docker image and start the container in detached mode
build-start: build
	$(DEV_DC) --profile $(PROFILE) up --remove-orphans -d

# Start the container in detached mode (NCTL only — unchanged)
start:
	$(DEV_DC) --profile $(PROFILE) up --remove-orphans -d

# Start the container and show logs in the terminal
start-log:
	$(DEV_DC) --profile $(PROFILE) up --remove-orphans

# Stop the container and clean up (NCTL profile only)
stop:
	$(DEV_DC) --profile $(PROFILE) down

# NCTL profile + MCP HTTP sidecar on :8788
start-all: start mcp-http

stop-all: stop mcp-http-stop

# Build / start / stop MCP sidecar only
mcp-build:
	$(DEV_DC) --profile mcp build nctl-mcp
	docker tag casper-nctl-2-docker-mcp:2.2.2 casper-nctl-2-docker-mcp:latest 2>/dev/null || true

mcp-http: mcp-build
	$(DEV_DC) --profile mcp up --remove-orphans -d nctl-mcp

mcp-http-stop:
	-docker stop casper-nctl-2-docker-mcp 2>/dev/null
	-docker rm casper-nctl-2-docker-mcp 2>/dev/null

# Host stdio MCP (Rust)
run-mcp:
	NCTL_DOCKER_ROOT="$(CURDIR)" cargo run --manifest-path mcp/Cargo.toml --quiet --

# Host HTTP MCP without Docker
run-mcp-http:
	NCTL_DOCKER_ROOT="$(CURDIR)" cargo run --manifest-path mcp/Cargo.toml --quiet -- --http --listen 127.0.0.1:8788

# Start the Docker container based on the specified profile (e.g. stable, 2.2, dev)
start-docker:
	docker run --rm -it ${IMAGE_NAME}:$(PROFILE)

# List of ports to expose
DOCKER_PORTS = \
	-p 11101-11105:11101-11105 \
	-p 14101-14105:14101-14105 \
	-p 18101-18105:18101-18105 \
	-p 25101-25105:25101-25105 \
	-p 28101-28105:28101-28105

# Common Docker volume mappings
DOCKER_VOLUMES = \
	-v ${PWD}/assets/faucet:/app/casper-nctl/assets/net-1/faucet \
	-v ${PWD}/assets/users:/app/casper-nctl/assets/net-1/users \
	-v ${PWD}/assets/chainspec:/app/casper-nctl/assets/net-1/chainspec \
	-v ${PWD}/assets/nodes:/app/casper-nctl/assets/net-1/nodes

# Run container based on passed version
define RUN_DOCKER
	docker run --rm -it \
		$(DOCKER_PORTS) \
		$(DOCKER_VOLUMES) \
		${IMAGE_NAME}:$1
endef

# Targets
start-docker-%:
	$(call RUN_DOCKER,$*)

# Catch-all rule for unrecognized make targets
%:
	@:

# Mark targets as not real files
.PHONY: build start build-start build-start-log start-docker-% \
	stop start-all stop-all mcp-build mcp-http mcp-http-stop run-mcp run-mcp-http
