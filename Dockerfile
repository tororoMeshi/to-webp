# ビルドステージ
FROM rust:1.86-slim AS builder

# 必要なライブラリをインストール
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev libwebp-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# 依存関係のキャッシュ層を作成
COPY Cargo.toml ./
RUN mkdir src && \
    echo "fn main() {println!(\"dummy\");}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 本来のソースコードをコピーしてビルド
COPY . .

# フォーマットとチェックを実行
RUN rustup component add clippy rustfmt && \
    cargo fmt && cargo check && cargo clippy -- -D warnings

# ビルド
RUN cargo build --release

# 実行環境（軽量イメージ）
FROM debian:12-slim

# 必要なランタイム依存関係をインストール
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates libwebp-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# ビルド済みバイナリをコピー
COPY --from=builder /usr/src/app/target/release/to-webp /app/

# 環境変数のデフォルト値設定
ENV HOST=0.0.0.0
ENV PORT=8080
ENV RUST_LOG=info

# 実行ユーザー設定（セキュリティ向上）
RUN useradd -ms /bin/bash appuser
USER appuser

# ポート公開と実行コマンド
EXPOSE 8080
CMD ["./to-webp"]