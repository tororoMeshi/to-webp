# WebP変換API テスト資料

このディレクトリには、WebP変換APIのテストに関連するファイルが含まれています。

## テスト用ファイル

- `makinamiPB021427-edit_TP_V4.jpg`: テストに使用したサンプル画像
- `test-pod.yaml`: テスト実行用の一時Podマニフェスト
- `service-nodeport.yaml`: テスト用のNodePortサービスマニフェスト

## テスト手順

1. まず、Kubernetes上にアプリケーションをデプロイ
   ```bash
   kubectl apply -f ../k8s/create_namespace.yaml
   kubectl apply -f ../k8s/deployment.yaml
   kubectl apply -f ../k8s/service.yaml
   ```

2. テスト用Podを作成
   ```bash
   kubectl apply -f test-pod.yaml
   ```

3. テストPodに画像をコピー
   ```bash
   kubectl cp makinamiPB021427-edit_TP_V4.jpg converter/test-pod:/tmp/
   ```

4. WebP変換APIを呼び出し
   ```bash
   kubectl exec -n converter test-pod -- curl -X POST -F "image=@/tmp/makinamiPB021427-edit_TP_V4.jpg" http://webp-converter/convert -o /tmp/converted.webp
   ```

5. 変換結果の確認
   ```bash
   kubectl exec -n converter test-pod -- du -h /tmp/makinamiPB021427-edit_TP_V4.jpg /tmp/converted.webp
   # サイズの確認: JPG（約60KB）→ WebP（約420KB、ロスレス変換）
   ```

6. テスト後のクリーンアップ
   ```bash
   kubectl delete pod test-pod -n converter
   ```