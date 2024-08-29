#!/bin/bash

# Save the current directory path
CURRENT_DIR=$(pwd)

# Navigate to the root directory of the project
cd ../../

# Define the Docker image name as a variable
DOCKER_IMAGE_NAME="luminainc/pdf2png"

# Get the current commit SHA
SHA=$(git rev-parse --short HEAD)
echo "------------------------"
echo $SHA
echo "------------------------"

# Build the Docker image with the SHA tag, using the correct path for the Dockerfile
docker build --platform linux/amd64 -t $DOCKER_IMAGE_NAME:$SHA -f services/pdf2png/Dockerfile .

# Check if the build was successful
if [ $? -eq 0 ]; then
    # Push the Docker image with the SHA tag
    docker push $DOCKER_IMAGE_NAME:$SHA

    # Optionally, you can also tag and push as latest
    docker tag $DOCKER_IMAGE_NAME:$SHA $DOCKER_IMAGE_NAME:latest
    docker push $DOCKER_IMAGE_NAME:latest
else
    echo "Docker build failed. Skipping push."
    exit 1
fi