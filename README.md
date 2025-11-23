# bskpost

`bskpost` は、コマンドラインから Bluesky にテキストを投稿するためのシンプルな Rust 製ツールです。

## 機能

*   Bluesky アカウントへの認証
*   テキストメッセージの投稿
*   設定ファイルによる認証情報の管理

## 必要要件

*   Rust (cargo) がインストールされていること

## インストールとビルド

このリポジトリをクローンし、以下のコマンドでビルドしてください。

```bash
git clone https://github.com/djanzu/bskpost.git
cd bskpost
cargo build --release
```

ビルドが完了すると、`target/release/bskpost` に実行ファイルが生成されます。

## 設定

ホームディレクトリに `.bskenv` という名前の設定ファイルを作成し、以下の形式で Bluesky のハンドルネームとアプリパスワードを記述してください。

**ファイルパス:** `~/.bskenv`

```env
BSK_HANDLE=your.handle.bsky.social
BSK_APP_PASS=your-app-password-here
```

*   `BSK_HANDLE`: あなたの Bluesky ハンドルネーム (例: `example.bsky.social`)
*   `BSK_APP_PASS`: Bluesky の設定画面で生成したアプリパスワード (通常のログインパスワードではなく、アプリパスワードの使用を推奨します)

## 使い方

引数に投稿したいテキストを指定して実行します。

```bash
./target/release/bskpost "Hello, Bluesky from CLI!"
```

または、標準入力からテキストを渡すことも可能です（パイプやリダイレクトを使用する場合）。

```bash
echo "Hello from stdin" | ./target/release/bskpost -
# または
cat post.txt | ./target/release/bskpost -
```

`--` を使用してオプションの終わりを明示することもできます。

```bash
./target/release/bskpost -- -
```

### 実行例

```bash
$ ./target/release/bskpost "これはテスト投稿です。"
Authenticating as example.bsky.social...
Posting message...
Successfully posted to Bluesky!
```

## 仕様詳細

*   **言語**: Rust (Edition 2021)
*   **依存クレート**:
    *   `reqwest`: HTTP クライアント
    *   `tokio`: 非同期ランタイム
    *   `serde`, `serde_json`: JSON 処理
    *   `clap`: コマンドライン引数のパース
    *   `dirs`: ホームディレクトリの特定
    *   `chrono`: タイムスタンプ処理
    *   `anyhow`: エラーハンドリング
*   **API**: AT Protocol (Bluesky)
    *   認証: `com.atproto.server.createSession`
    *   投稿: `com.atproto.repo.createRecord` (`app.bsky.feed.post`)

## ライセンス

MIT License
