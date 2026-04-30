# PROGRESS.md — Rust チャットサーバー 学習進捗

## 現在のフェーズ
- [x] Phase 1: 同期版 土台（完了）
- [x] Phase 2: 同期版 マルチスレッド化（完了）
- [ ] Phase 3: 非同期の概念理解
- [ ] Phase 4: Tokio版への書き換え
- [ ] Phase 5: 仕上げ

## 直近の作業
Phase 2 全ステップ完了: マルチスレッドチャットサーバー（Arc/Mutex/mpsc・コマンド・入退室通知・LAN公開）動作確認済み

## 次にやること
Phase 3 Step 10: Rust Book Ch.17 を読んで Future・Poll の概念を整理

## 詰まっていること・メモ
（エラー内容や疑問点があればここに書く）

---

## 完了済みステップ

### Phase 1: 同期版 土台
- [x] Step 1: `cargo new chat-server --bin` でプロジェクト作成
- [x] Step 2: `TcpListener::bind` で1対1接続の受け付け
- [x] Step 3: `BufReader` / `BufWriter` で行単位の読み書き
- [x] Step 4: `enum Message` と `match` でコマンド処理

### Phase 2: 同期版 マルチスレッド化
- [x] Step 5: `thread::spawn` + `move` クロージャでスレッド起動
- [x] Step 6: `Arc<Mutex<HashMap<SocketAddr, Sender>>>` で共有状態管理
- [x] Step 7: `std::sync::mpsc` でブロードキャスト実装
- [x] Step 8: `/nick`・`/list`・`/quit` コマンド実装・入退室通知
- [x] Step 9: バインドアドレスを `0.0.0.0:8080` にして LAN 公開

### Phase 3: 非同期の概念理解
- [ ] Step 10: Rust Book Ch.17 を読んで Future・Poll の概念を整理
- [ ] Step 11: `async fn` / `.await` を小さなコードで試す
- [ ] Step 12: `tokio = { features = ["full"] }` を Cargo.toml に追加
- [ ] Step 13: `#[tokio::main]` でエントリポイントを作成
- [ ] Step 14: `tokio::time::sleep` と `thread::sleep` の違いを体感

### Phase 4: Tokio版への書き換え
- [ ] Step 15: `tokio::net::TcpListener` + `.accept().await` に置き換え
- [ ] Step 16: `tokio::spawn` で非同期タスク起動
- [ ] Step 17: `tokio::sync::broadcast::channel` でブロードキャスト実装
- [ ] Step 18: `AsyncBufReadExt` / `AsyncWriteExt` でストリーム読み書き
- [ ] Step 19: `tokio::sync::Mutex` vs `std::sync::Mutex` の使い分け

### Phase 5: 仕上げ
- [ ] Step 20: `tokio::select!` で受信と切断検知を同時に待機
- [ ] Step 21: `tokio::signal::ctrl_c()` でグレースフルシャットダウン
- [ ] Step 22: `tracing` + `tracing-subscriber` でロギング
- [ ] Step 23: `#[tokio::test]` でユニットテスト実装

---

## 次のセッション開始時
Copilot に「続きからお願いします」と伝えるだけでOK。
このファイルと `/memories/repo/project.md` を読んで再開します。
