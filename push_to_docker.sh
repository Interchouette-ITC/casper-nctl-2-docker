#!/bin/bash

DOCKER_REPO="gregoshop/casper-nctl"
TAGS=("2.0" "2.1" "2.2" "dev" "stable")
IMAGE_PREFIX="casper-nctl-2-docker"

# Loop through the TAGS array
for TAG in "${TAGS[@]}"; do
    # Compose builds images as casper-nctl-2-docker-<profile>:latest
    IMAGE_REF="${IMAGE_PREFIX}-${TAG}:latest"
    IMAGE_ID=$(docker images --format "{{.ID}}" --filter=reference="$IMAGE_REF" | head -n 1)

    # Check if IMAGE_ID is not empty
    if [ -z "$IMAGE_ID" ]; then
        echo "No image found with the name $IMAGE_REF"
        exit 1
    fi

    # Tag the image
    echo "Tagging image $IMAGE_ID as $DOCKER_REPO:$TAG"
    docker tag $IMAGE_ID $DOCKER_REPO:$TAG

    # Login to Docker Hub
    echo "Logging in to Docker Hub..."
    docker login || { echo 'Docker login failed'; exit 1; }

    # Push the image to Docker Hub
    echo "Pushing $DOCKER_REPO:$TAG to Docker Hub"
    docker push $DOCKER_REPO:$TAG

    # Check if push was successful
    if [ $? -eq 0 ]; then
        echo "Image $DOCKER_REPO:$TAG pushed successfully!"
    else
        echo "Failed to push image $DOCKER_REPO:$TAG."
    fi
done
