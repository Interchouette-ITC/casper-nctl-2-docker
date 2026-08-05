# Define the current directory and Docker Compose command
CURRENT_DIR = .
DEV_DC = docker compose -f $(CURRENT_DIR)/docker-compose.yml

# Extract the second argument from MAKECMDGOALS, defaulting to "stable" if not specified
PROFILE ?= $(word 2,$(MAKECMDGOALS))
PROFILE := $(if $(PROFILE),$(PROFILE),stable)

# NCTL image
IMAGE_NAME=interchouette/casper-nctl-2-docker

# MCP sidecar image (release line: 2.2 + latest; tip: :dev)
MCP_NAME=casper-nctl-2-docker-mcp
MCP_HUB=interchouette/casper-nctl-2-docker-mcp
MCP_GHCR_PERSONAL=ghcr.io/groussac/casper-nctl-2-docker-mcp
MCP_GHCR_ORG=ghcr.io/interchouette-itc/casper-nctl-2-docker-mcp
MCP_TAG=2.2

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

# Start the container in detached mode (NCTL only)
start:
	$(DEV_DC) --profile $(PROFILE) up --remove-orphans -d

# Start the container and show logs in the terminal
start-log:
	$(DEV_DC) --profile $(PROFILE) up --remove-orphans

# Stop the container and clean up (NCTL profile only)
stop:
	$(DEV_DC) --profile $(PROFILE) down

# NCTL profile + MCP HTTP sidecar on :8791
start-all: start mcp-http

stop-all: stop mcp-http-stop

# --- MCP sidecar ---

mcp-build:
	docker build -t $(MCP_NAME):$(MCP_TAG) -t $(MCP_NAME):latest \
		-t $(MCP_HUB):$(MCP_TAG) -t $(MCP_HUB):latest \
		-f mcp/Dockerfile mcp

mcp-build-dev:
	docker build -t $(MCP_NAME):dev -t $(MCP_HUB):dev \
		-t $(MCP_GHCR_PERSONAL):dev -t $(MCP_GHCR_ORG):dev \
		-f mcp/Dockerfile mcp

# Prefer Hub image; build locally if pull fails
mcp-http:
	-docker pull $(MCP_HUB):$(MCP_TAG)
	$(DEV_DC) --profile mcp up --remove-orphans -d nctl-mcp

mcp-http-stop:
	-docker stop casper-nctl-2-docker-mcp 2>/dev/null
	-docker rm casper-nctl-2-docker-mcp 2>/dev/null

# Host stdio / HTTP MCP (needs Rust toolchain)
run-mcp:
	NCTL_DOCKER_ROOT="$(CURDIR)" cargo run --manifest-path mcp/Cargo.toml --quiet --

run-mcp-http:
	NCTL_DOCKER_ROOT="$(CURDIR)" cargo run --manifest-path mcp/Cargo.toml --quiet -- --http --listen 127.0.0.1:8791

mcp-docker-push-hub:
	docker push $(MCP_HUB):$(MCP_TAG)
	docker push $(MCP_HUB):latest

mcp-docker-push-ghcr-personal:
	docker tag $(MCP_HUB):$(MCP_TAG) $(MCP_GHCR_PERSONAL):$(MCP_TAG)
	docker tag $(MCP_HUB):latest $(MCP_GHCR_PERSONAL):latest
	docker push $(MCP_GHCR_PERSONAL):$(MCP_TAG)
	docker push $(MCP_GHCR_PERSONAL):latest

mcp-docker-push-ghcr-itc:
	docker tag $(MCP_HUB):$(MCP_TAG) $(MCP_GHCR_ORG):$(MCP_TAG)
	docker tag $(MCP_HUB):latest $(MCP_GHCR_ORG):latest
	docker push $(MCP_GHCR_ORG):$(MCP_TAG)
	docker push $(MCP_GHCR_ORG):latest

mcp-docker-push-dev-hub:
	docker push $(MCP_HUB):dev

mcp-docker-push-dev-ghcr-personal:
	docker push $(MCP_GHCR_PERSONAL):dev

mcp-docker-push-dev-ghcr-itc:
	docker push $(MCP_GHCR_ORG):dev

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
	stop start-all stop-all \
	mcp-build mcp-build-dev mcp-http mcp-http-stop run-mcp run-mcp-http \
	mcp-docker-push-hub mcp-docker-push-ghcr-personal mcp-docker-push-ghcr-itc \
	mcp-docker-push-dev-hub mcp-docker-push-dev-ghcr-personal mcp-docker-push-dev-ghcr-itc
