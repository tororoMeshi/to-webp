# Rustの安定バージョンを使用
FROM rust:1.86-slim

# 必要なコンポーネントやパッケージをインストール
RUN rustup component add clippy rustfmt \
    && apt-get update \
    && apt-get install -y pkg-config libssl-dev libwebp-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# ソースコード全体をコピー
COPY . .

# フォーマットとチェックを実行
RUN cargo fmt && cargo check && cargo clippy -- -D warnings

# ビルド
RUN cargo build --release

# 実行用の設定
EXPOSE 8080
CMD ["./target/release/to-webp"]