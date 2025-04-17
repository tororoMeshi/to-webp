#!/bin/bash
# This script builds a Docker image and pushes it to Docker Hub.
# Usage: ./push_to_dockerhub.sh [IMAGE_TAG]
# Make sure you are logged in to Docker Hub before running this script.

set -eu

# オプションの処理
IMAGE_TAG=$(date +%Y%m%d%H%M)

# 引数の処理
if [ $# -ge 1 ]; then
  IMAGE_TAG="$1"
fi

# イメージ名とタグを設定
IMAGE_NAME="tororomeshi/to-webp"

# スクリプトディレクトリを取得
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "${SCRIPT_DIR}"

# Cargo.lockファイルがあれば削除（Dockerビルド内で依存関係を解決する）
if [ -f "Cargo.lock" ]; then
  echo "Removing Cargo.lock to let Docker handle dependencies..."
  rm Cargo.lock
fi

# Dockerイメージをビルド
echo "Building Docker image..."
if ! docker build -t "${IMAGE_NAME}:${IMAGE_TAG}" -t "${IMAGE_NAME}:latest" .; then
  echo "Docker build failed." >&2
  exit 1
fi

# Function to push image and handle authentication errors
push_image() {
  local TAG=$1
  echo "Pushing Docker image with tag ${TAG}..."
  if ! docker push "${IMAGE_NAME}:${TAG}"; then
    echo "Docker push failed for tag ${TAG}." >&2
    echo "Please make sure you are logged in to Docker Hub by running 'docker login'." >&2
    exit 1
  fi
}

# Push image with specific tag
push_image "${IMAGE_TAG}"

# Push image with latest tag
push_image "latest"

echo "Docker image pushed successfully to: ${IMAGE_NAME}:${IMAGE_TAG}"
echo "Also tagged as: ${IMAGE_NAME}:latest"