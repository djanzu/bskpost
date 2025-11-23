use anyhow::{Context, Result, bail};
use std::fs;

pub struct Config {
    pub handle: String,
    pub app_pass: String,
}

pub fn load_config() -> Result<Config> {
    let home_dir = dirs::home_dir().context("ホームディレクトリが見つかりませんでした")?;
    let config_path = home_dir.join(".bskenv");

    if !config_path.exists() {
        bail!("設定ファイルが見つかりません: {:?}", config_path);
    }

    let content = fs::read_to_string(&config_path)
        .context(".bskenvファイルの読み込みに失敗しました")?;

    let mut handle = None;
    let mut app_pass = None;

    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "BSK_HANDLE" => handle = Some(value.to_string()),
                "BSK_APP_PASS" => app_pass = Some(value.to_string()),
                _ => {}
            }
        }
    }

    let handle = handle.context(".bskenvにBSK_HANDLEが見つかりません")?;
    let app_pass = app_pass.context(".bskenvにBSK_APP_PASSが見つかりません")?;

    Ok(Config { handle, app_pass })
}
