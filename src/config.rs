use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};


pub struct Config {
    pub handle: String,
    pub app_pass: String,
}

pub fn load_config() -> Result<Config> {
    let home_dir = dirs::home_dir().context("Could not find home directory")?;
    let config_path = home_dir.join(".bskenv");

    let file = File::open(&config_path)
        .with_context(|| format!("Could not open config file at {:?}", config_path))?;
    let reader = BufReader::new(file);

    let mut handle = None;
    let mut app_pass = None;

    for line in reader.lines() {
        let line = line?;
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim().to_string();
            match key {
                "BSK_HANDLE" => handle = Some(value),
                "BSK_APP_PASS" => app_pass = Some(value),
                _ => {}
            }
        }
    }

    Ok(Config {
        handle: handle.context("BSK_HANDLE not found in .bskenv")?,
        app_pass: app_pass.context("BSK_APP_PASS not found in .bskenv")?,
    })
}
