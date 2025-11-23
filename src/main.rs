mod bsky;
mod config;

use anyhow::{Result, bail, Context};
use clap::Parser;
use crate::bsky::BskyClient;
use crate::config::load_config;
use std::io::{self, Read, IsTerminal};

// Blueskyの文字数制限（目安）
const MAX_CHARS: usize = 300;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The text content of the post. If not provided or "-", reads from stdin.
    #[arg(index = 1)]
    text: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    let text = match args.text.as_deref() {
        Some("-") | None => {
            if std::io::stdin().is_terminal() && args.text.is_none() {
                 bail!("エラー: 投稿内容が指定されていません。引数で指定するか、標準入力から渡してください。");
            }
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).context("標準入力からの読み込みに失敗しました")?;
            buffer
        }
        Some(s) => s.to_string(),
    };

    // --- 追加したバリデーション（検証）ロジック ---

    // 1. 空文字または空白のみのチェック
    if text.trim().is_empty() {
        bail!("エラー: 投稿内容が空です。テキストを入力してください。");
    }

    // 2. 文字数カウント（Unicode文字数としてカウント）
    let char_count = text.chars().count();
    if char_count > MAX_CHARS {
        bail!(
            "エラー: 文字数が制限を超えています。現在の文字数: {} (上限: {})", 
            char_count, 
            MAX_CHARS
        );
    }

    // -------------------------------------------
    
    // 設定を読み込む
    let config = load_config()?;
    
    println!("{} として認証中...", config.handle);
    let client = BskyClient::authenticate(&config.handle, &config.app_pass).await?;
    
    println!("投稿中...");
    client.create_post(&text).await?;
    
    println!("Blueskyへの投稿に成功しました！");
    
    Ok(())
}
