# Define the current directory and Docker Compose command
CURRENT_DIR = .
DEV_DC = docker compose -f $(CURRENT_DIR)/docker-compose.yml

# Extract the second argument from MAKECMDGOALS, defaulting to "stable" if not specified
PROFILE ?= $(word 2,$(MAKECMDGOALS))
PROFILE := $(if $(PROFILE),$(PROFILE),stable)

# Define the image name
IMAGE_NAME=gregoshop/casper-nctl

# 🏗️ Build the Docker image for the specified profile (default is stable)
build:
	$(DEV_DC) --profile $(PROFILE) build

# 🏗️ Build the Docker image without using cache (for a fresh build)
build-no-cache:
	$(DEV_DC) --profile $(PROFILE) build --no-cache

# 🔄 Build the Docker image and start the container with logs
build-start-log: build-no-cache
	$(DEV_DC) --profile $(PROFILE) up --remove-orphans

# 🚀 Build the Docker image and start the container in detached mode
build-start: build
	$(DEV_DC) --profile $(PROFILE) up --remove-orphans -d

# ▶️ Start the container in detached mode
start:
	$(DEV_DC) --profile $(PROFILE) up --remove-orphans -d

# 📝 Start the container and show logs in the terminal
start-log:
	$(DEV_DC) --profile $(PROFILE) up --remove-orphans

# ❌ Stop the container and clean up
stop:
	$(DEV_DC) --profile $(PROFILE) down

# 🔧 Start the Docker container based on the specified profile (e.g., stable, dev, 2.0)
start-docker:
	docker run --rm -it ${IMAGE_NAME}:$(PROFILE)

# 🔧 Start the Docker container for the stable version
start-docker-stable:
	docker run --rm -it ${IMAGE_NAME}:stable

# 🔧 Start the Docker container for the dev version
start-docker-dev:
	docker run --rm -it ${IMAGE_NAME}:dev

# 🔧 Start the Docker container for the 2.0 version
start-docker-2.0:
	docker run --rm -it ${IMAGE_NAME}:2.0

# Catch-all rule for unrecognized make targets
%:
	@:

# Mark targets as not real files
.PHONY: build start build-start build-start-log start-docker start-docker-stable start-docker-dev start-docker-2.0
