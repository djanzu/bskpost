use anyhow::{Context, Result};
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize)]
struct AuthRequest<'a> {
    identifier: &'a str,
    password: &'a str,
}

#[derive(Deserialize)]
struct AuthResponse {
    #[serde(rename = "accessJwt")]
    access_jwt: String,
    did: String,
}

pub struct BskyClient {
    client: Client,
    access_jwt: String,
    did: String,
}

impl BskyClient {
    pub async fn authenticate(handle: &str, app_pass: &str) -> Result<Self> {
        let client = Client::new();
        let resp = client
            .post("https://bsky.social/xrpc/com.atproto.server.createSession")
            .json(&AuthRequest {
                identifier: handle,
                password: app_pass,
            })
            .send()
            .await
            .context("認証リクエストの送信に失敗しました")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("認証に失敗しました: {} - {}", status, text);
        }

        let auth_data: AuthResponse = resp
            .json()
            .await
            .context("認証レスポンスの解析に失敗しました")?;

        Ok(Self {
            client,
            access_jwt: auth_data.access_jwt,
            did: auth_data.did,
        })
    }

    pub async fn create_post(&self, text: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        
        let payload = json!({
            "repo": self.did,
            "collection": "app.bsky.feed.post",
            "record": {
                "text": text,
                "createdAt": now,
                "$type": "app.bsky.feed.post" 
            }
        });

        let resp = self.client
            .post("https://bsky.social/xrpc/com.atproto.repo.createRecord")
            .header("Authorization", format!("Bearer {}", self.access_jwt))
            .json(&payload)
            .send()
            .await
            .context("投稿リクエストの送信に失敗しました")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("投稿の作成に失敗しました: {} - {}", status, text);
        }

        Ok(())
    }
}
