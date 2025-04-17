# to-webp 🖼️

ロスレスWebP変換APIサービス - 画像をステートレスに高速WebP変換

## 📘 概要

**to-webp**は、アップロードした画像を即座にロスレスWebP形式に変換して返すRust製APIサービスです。サーバーには画像データを保持せず、ステートレスな運用が可能です。

- ロスレス変換：画質を保持したまま変換
- ステートレス設計：サーバーに画像を保存しない
- 軽量・高速：Rustで実装された高性能API
- Docker/K8s対応：どこでも簡単にデプロイ可能

## 🔌 API仕様

### 画像変換エンドポイント

```
POST /convert
Content-Type: multipart/form-data
```

**パラメータ:**
- `image`: 変換する画像ファイル（PNG, JPEG, GIF, BMP対応）

**成功レスポンス:**
- ステータス: 200 OK
- Content-Type: image/webp
- ボディ: WebP画像バイナリ

**エラーレスポンス:**
- JSON形式のエラー情報
```json
{
  "error": "error_code",
  "message": "エラーメッセージ",
  "hint": "対処方法の提案"
}
```

## 🚀 使用方法

### Dockerで実行

```bash
# イメージビルド・起動
docker-compose up -d

# 使用例（curlコマンド）
curl -X POST -F "image=@sample.jpg" http://localhost:8080/convert -o converted.webp
```

### ローカル開発環境

```bash
# 必要なライブラリインストール
apt-get install -y libwebp-dev

# 依存関係のインストールとビルド
cargo build

# 実行
cargo run

# テスト
cargo test
```

## 📦 デプロイ

Kubernetesマニフェストが`k8s/`ディレクトリに用意されています：

```bash
# デプロイ
kubectl apply -f k8s/

# スケーリング例
kubectl scale deployment webp-converter --replicas=5
```

## 🔐 制約事項

- 最大ファイルサイズ: 10MB
- サポート形式: PNG, JPEG, GIF, BMP
- 非サポート形式: HEIC, HEIF