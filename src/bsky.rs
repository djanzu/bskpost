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
            .context("Failed to send auth request")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Authentication failed: {} - {}", status, text);
        }

        let auth_data: AuthResponse = resp
            .json()
            .await
            .context("Failed to parse auth response")?;

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
            .context("Failed to send create record request")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Failed to create post: {} - {}", status, text);
        }

        Ok(())
    }
}
