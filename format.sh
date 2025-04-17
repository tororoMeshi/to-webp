#!/bin/bash
set -eux

# コードをフォーマット
cargo fmt

# Dockerコンテナでコードのチェックを実行
docker build -t rust-check -f rust-check .
docker run --rm rust-check