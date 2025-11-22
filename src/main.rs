mod bsky;
mod config;

use anyhow::Result;
use clap::Parser;
use crate::bsky::BskyClient;
use crate::config::load_config;

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
    
    // Load configuration
    let config = load_config()?;
    
    println!("Authenticating as {}...", config.handle);
    let client = BskyClient::authenticate(&config.handle, &config.app_pass).await?;
    
    println!("Posting message...");
    client.create_post(&args.text).await?;
    
    println!("Successfully posted to Bluesky!");
    
    Ok(())
}
