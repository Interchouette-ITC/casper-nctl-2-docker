#!/bin/bash

DOCKER_REPOS=("gregoshop/casper-nctl" "interchouette/casper-nctl")
TAGS=("1.5.8" "stable" "2.2" "dev")
IMAGE_PREFIX="casper-nctl-2-docker"

# Login once
echo "Logging in to Docker Hub..."
docker login || { echo 'Docker login failed'; exit 1; }

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

    for DOCKER_REPO in "${DOCKER_REPOS[@]}"; do
        echo "Tagging image $IMAGE_ID as $DOCKER_REPO:$TAG"
        docker tag $IMAGE_ID $DOCKER_REPO:$TAG

        echo "Pushing $DOCKER_REPO:$TAG to Docker Hub"
        docker push $DOCKER_REPO:$TAG

        if [ $? -eq 0 ]; then
            echo "Image $DOCKER_REPO:$TAG pushed successfully!"
        else
            echo "Failed to push image $DOCKER_REPO:$TAG."
            exit 1
        fi
    done
done
