mod bsky;
mod config;

use anyhow::{Result, bail}; // bailマクロを追加
use clap::Parser;
use crate::bsky::BskyClient;
use crate::config::load_config;

// Blueskyの文字数制限（目安）
const MAX_CHARS: usize = 300;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The text content of the post
    #[arg(index = 1)]
    text: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // --- 追加したバリデーション（検証）ロジック ---

    // 1. 空文字または空白のみのチェック
    if args.text.trim().is_empty() {
        bail!("エラー: 投稿内容が空です。テキストを入力してください。");
    }

    // 2. 文字数カウント（Unicode文字数としてカウント）
    let char_count = args.text.chars().count();
    if char_count > MAX_CHARS {
        bail!(
            "エラー: 文字数が制限を超えています。現在の文字数: {} (上限: {})", 
            char_count, 
            MAX_CHARS
        );
    }

    // -------------------------------------------
    
    // Load configuration
    let config = load_config()?;
    
    println!("Authenticating as {}...", config.handle);
    let client = BskyClient::authenticate(&config.handle, &config.app_pass).await?;
    
    println!("Posting message...");
    client.create_post(&args.text).await?;
    
    println!("Successfully posted to Bluesky!");
    
    Ok(())
}
