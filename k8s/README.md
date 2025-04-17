# to-webp Kubernetes デプロイ

WebP変換APIをKubernetesにデプロイするための手順です。内部サービスとして動作します。

## セットアップ手順

### 1. namespaceの作成

```bash
kubectl apply -f create_namespace.yaml
```

### 2. アプリケーションのデプロイ

```bash
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml
```

## リソースの確認

```bash
# Pod状態の確認
kubectl get pods -n converter

# Serviceの確認
kubectl get svc -n converter
```

## スケーリング

必要に応じてレプリカ数を調整できます：

```bash
kubectl scale deployment webp-converter -n converter --replicas=5
```

## トラブルシューティング

### ログの確認

```bash
# Podのログを表示
kubectl logs -n converter -l app=webp-converter
```

### 動作確認

```bash
# Podに接続してヘルスチェック
kubectl exec -it $(kubectl get pods -n converter -l app=webp-converter -o jsonpath="{.items[0].metadata.name}") -n converter -- curl localhost:8080/health

# サービス経由でのアクセステスト（クラスター内から）
kubectl run -it --rm --restart=Never curl-test --image=curlimages/curl -n converter -- curl webp-converter.converter.svc.cluster.local/health
```