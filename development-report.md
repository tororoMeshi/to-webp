# to-webp プロジェクト開発レポート

## 🔍 プロジェクト概要

「to-webp」は、アップロードされた画像（PNG, JPEG, GIF, BMP）をロスレスWebP形式に変換するREST APIサービスです。主な特徴:

- ステートレス設計（サーバー側に画像を保存しない）
- ロスレス変換によって高品質なWebP画像を生成
- Rust + actix-webフレームワークで実装された高速な処理
- Docker + Kubernetesによるコンテナ化とデプロイ対応

## 📝 開発フロー

### 1. 設計フェーズ

`system_design.md` に基づき、以下の仕様を確認:

- エンドポイント `/convert` でマルチパートフォームデータを受け取る
- 最大10MBまでの画像サイズに対応
- HEIC/HEIF形式は非対応、適切なエラーメッセージを返す
- 成功時はバイナリのWebPデータを返却
- エラー時はJSONフォーマットでエラー情報を返却

### 2. 実装フェーズ

#### 2.1 基本構造の構築

- アプリケーション用のプロジェクト構造を作成
  - `Cargo.toml`に必要な依存関係を追加
  - `src/main.rs`に主要なコード実装
  - エラー処理と適切なレスポンス定義

#### 2.2 機能実装

- マルチパートフォームデータのパース
- ファイル形式のバリデーション
- WebPへの変換処理
- エラーハンドリングと適切なHTTPステータスコード
- ヘルスチェックエンドポイント

#### 2.3 コンテナ化

- マルチステージビルドを使用したDockerfile作成
  - ビルド環境と実行環境の分離によるイメージサイズ削減
  - 依存関係キャッシュ層の最適化
- Docker Hub向けプッシュスクリプト作成

#### 2.4 Kubernetes対応

- `converter` namespaceを使用
- Deployment, Service, ConfigMapの作成
- リソース制限の設定
- ヘルスチェックプローブの設定

### 3. テストフェーズ

- 単体テストの実装（`src/tests.rs`）
- Kubernetesクラスター上での実際の動作テスト
  - テスト用コンテナからのAPI呼び出し
  - JPG→WebP変換の確認
  - ファイルサイズと変換品質の確認

### 4. CI/CD対応

- DockerイメージをDocker Hubに公開
- ビルド・テスト自動化のためのスクリプト作成
- 開発ワークフローの整備と文書化

## 🛠️ 使用技術

1. **バックエンド**: 
   - Rust 1.86
   - actix-web: Webフレームワーク
   - image, webp: 画像処理・変換
   - serde: シリアライズ/デシリアライズ

2. **コンテナ/インフラ**:
   - Docker: マルチステージビルド
   - Kubernetes: デプロイメント管理
   - Docker Hub: イメージレジストリ

3. **その他**:
   - GitHub: ソースコード管理
   - Cargo: ビルド・依存関係管理

## 📊 プロジェクト構造

```
to-webp/
├── Cargo.toml               # Rust依存関係
├── CLAUDE.md                # AI開発アシスタント用設定
├── Dockerfile               # マルチステージビルド設定
├── README.md                # プロジェクト概要
├── development-workflow.md  # 開発ワークフロー
├── format.sh                # コードフォーマット
├── k8s/                     # Kubernetes設定
│   ├── README.md
│   ├── create_namespace.yaml
│   ├── deployment.yaml
│   └── service.yaml
├── push_to_dockerhub.sh     # Docker Hubプッシュスクリプト
├── rust-check               # Rustリント用Dockerfile
├── src/                     # ソースコード
│   ├── main.rs              # アプリケーションのメインコード
│   └── tests.rs             # テストコード
└── tests/                   # テスト資料
    ├── README.md
    ├── makinamiPB021427-edit_TP_V4.jpg
    ├── service-nodeport.yaml
    └── test-pod.yaml
```

## 🚀 主要成果

1. **高速なWebP変換API**:
   - JPG, PNG, GIF, BMPからWebPへのロスレス変換を即時実行
   - 平均変換時間: 数百ミリ秒以内（小〜中サイズ画像）

2. **最適化されたコンテナ**:
   - マルチステージビルドによるイメージサイズ削減
   - 非rootユーザー実行によるセキュリティ向上

3. **完全なK8s対応**:
   - ステートレス設計によるスケールアウト容易性
   - ヘルスチェック・リソース制限の設定

4. **整備された開発ワークフロー**:
   - コード品質確認のための自動化
   - CI/CDパイプライン対応準備

## 🔄 発生した問題と対応

1. **Dockerfile最適化**:
   - **問題**: 初期のDockerfileは最適化されておらず、イメージサイズが大きかった
   - **解決**: マルチステージビルドを採用し、依存関係のキャッシュ層も改善

2. **API動作検証**:
   - **問題**: Kubernetes環境での直接APIテストが複雑
   - **解決**: 一時的なテストPodを作成し、そこからAPI呼び出しを実行

3. **依存関係の解決**:
   - **問題**: `mime_guess`のインポートや`WebPMemory`の扱いに関する問題
   - **解決**: 適切なクレートの追加と型変換の実装

## 📈 改善点・今後の展開

1. **性能最適化**:
   - 画像サイズによる変換パラメータの動的調整
   - WebPオプション（ロッシー・ロスレス）の選択機能

2. **機能拡張**:
   - サイズ変更オプションの追加
   - リサイズとトリミング機能の実装
   - バッチ処理による複数ファイルの一括変換

3. **運用改善**:
   - Prometheus/Grafanaによるモニタリング導入
   - 自動スケーリング対応の強化
   - APIドキュメントの自動生成

4. **セキュリティ強化**:
   - より厳密な入力バリデーション
   - レート制限の実装
   - セキュリティスキャンの導入

## 🏁 まとめ

to-webpプロジェクトは、Rustを使用した高速なWebP変換APIの実装とコンテナ化を達成しました。設計からテスト・デプロイまでの一連のプロセスを通じて、高品質なコードベースとデプロイフローを確立しました。開発ワークフローの整備によって、今後の機能追加やメンテナンスも容易になっています。

本プロジェクトは、Rust言語のパフォーマンスとactix-webフレームワークの効率性を活かした実装例として、他の高性能APIサービス開発の参考にもなるでしょう。

---

## 📋 効率的な開発のためのファイルサンプル

### 1. 開発ステップチェックリスト (project-steps.md)

```markdown
# WebP変換API 開発ステップチェックリスト

## 設計フェーズ
- [ ] 要件文書のレビューと確認
- [ ] API仕様の策定（エンドポイント、リクエスト/レスポンス形式）
- [ ] エラーハンドリング方針の決定
- [ ] 性能目標の設定

## 環境セットアップ
- [ ] Rust環境のセットアップ
- [ ] 必要なライブラリの確認
- [ ] Docker環境のセットアップ
- [ ] Kubernetes環境のセットアップ

## 実装フェーズ
- [ ] プロジェクト骨格の作成
- [ ] エラー型の定義
- [ ] マルチパートフォームのパース処理
- [ ] 画像形式のバリデーション
- [ ] WebP変換のコア機能実装 
- [ ] ヘルスチェックエンドポイント実装
- [ ] ロギング実装

## テストフェーズ
- [ ] 単体テストの作成
- [ ] 変換品質テスト（サンプル画像準備）
- [ ] エラーケーステスト
- [ ] 大サイズ画像テスト
- [ ] 不正形式テスト

## コンテナ化・デプロイ
- [ ] Dockerfileの作成
- [ ] マルチステージビルドの最適化
- [ ] Docker Hub連携
- [ ] Kubernetesマニフェスト作成
- [ ] デプロイテスト

## 文書化
- [ ] README作成
- [ ] API仕様書作成
- [ ] デプロイ手順書作成
- [ ] 開発レポート作成
```

### 2. Rust WebAPI テンプレート構造

```
rust-webapi-template/
├── Cargo.toml               # 依存関係定義（通常よく使われるものを事前定義）
├── rust-toolchain.toml      # Rustバージョン固定（安定性向上）
├── .env.example             # 環境変数サンプル（設定しやすさ向上） 
├── src/
│   ├── main.rs              # エントリーポイント
│   ├── config.rs            # 設定読み込み
│   ├── error.rs             # エラー型定義
│   ├── handlers/            # API ハンドラ
│   │   ├── mod.rs
│   │   └── health.rs        # ヘルスチェック 
│   ├── models/              # データモデル
│   │   └── mod.rs
│   ├── services/            # ビジネスロジック
│   │   └── mod.rs
│   └── utils/               # ユーティリティ
│       └── mod.rs
├── tests/                   # テスト
│   ├── common.rs            # テスト共通機能
│   ├── api_tests.rs         # API統合テスト
│   └── unit_tests.rs        # 単体テスト 
├── docker/                  # Docker関連
│   ├── Dockerfile           # 本番用Dockerfile
│   └── Dockerfile.dev       # 開発用Dockerfile
└── k8s/                     # Kubernetes マニフェスト
    ├── deployment.yaml
    ├── service.yaml
    └── configmap.yaml
```

### 3. 開発・デプロイフロー自動化スクリプト (dev.sh)

```bash
#!/bin/bash
set -e

# 色付き出力
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

function show_help {
  echo -e "${BLUE}WebP API 開発スクリプト${NC}"
  echo ""
  echo "使用方法: ./dev.sh [コマンド]"
  echo ""
  echo "コマンド:"
  echo "  format        コードのフォーマットとリントを実行"
  echo "  test          テストを実行"
  echo "  build         ローカルでビルド"
  echo "  docker        Dockerイメージをビルド"
  echo "  push          Docker Hubにプッシュ"
  echo "  deploy        Kubernetesにデプロイ"
  echo "  check-all     全ての検証を実行"
  echo "  help          このヘルプを表示"
}

function format {
  echo -e "${BLUE}コードのフォーマットとリントを実行中...${NC}"
  cargo fmt
  cargo clippy -- -D warnings
  echo -e "${GREEN}フォーマット完了${NC}"
}

function run_tests {
  echo -e "${BLUE}テストを実行中...${NC}"
  cargo test
  echo -e "${GREEN}テスト完了${NC}"
}

function build {
  echo -e "${BLUE}ローカルでビルド中...${NC}"
  cargo build --release
  echo -e "${GREEN}ビルド完了${NC}"
}

function docker_build {
  echo -e "${BLUE}Dockerイメージをビルド中...${NC}"
  docker build -t tororomeshi/to-webp:latest .
  echo -e "${GREEN}Dockerイメージのビルド完了${NC}"
}

function docker_push {
  echo -e "${BLUE}Docker Hubにプッシュ中...${NC}"
  ./push_to_dockerhub.sh
  echo -e "${GREEN}プッシュ完了${NC}"
}

function deploy {
  echo -e "${BLUE}Kubernetesにデプロイ中...${NC}"
  kubectl apply -f k8s/create_namespace.yaml
  kubectl apply -f k8s/deployment.yaml
  kubectl apply -f k8s/service.yaml
  echo -e "${GREEN}デプロイ完了${NC}"
}

function check_all {
  format
  run_tests
  docker_build
}

# コマンドライン引数の処理
case "$1" in
  format)
    format
    ;;
  test)
    run_tests
    ;;
  build)
    build
    ;;
  docker)
    docker_build
    ;;
  push)
    docker_push
    ;;
  deploy)
    deploy
    ;;
  check-all)
    check_all
    ;;
  help|*)
    show_help
    ;;
esac
```

これらのファイルがあれば、開発のステップや構成が最初から明確になり、手順の前後関係もわかりやすくなります。特に新規開発者がプロジェクトに参加する際も、これらのファイルがあることで迅速に開発環境と手順を理解できるでしょう。