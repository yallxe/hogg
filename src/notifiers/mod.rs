use std::collections::HashMap;

use crate::{scanner::NucleiJsonOutput, config::Config};
use notify_rust::Notification;
use anyhow::Result;

pub async fn scanner_notify(data: NucleiJsonOutput, config: &Config) -> Result<()> {
    Notification::new()
        .summary("Hogg")
        .body("New vulnerability found!")
        .show()?;
    
    send_telegram(&data, config).await?;
    Ok(())
}

pub async fn send_telegram(data: &NucleiJsonOutput, config: &Config) -> Result<()> {
    let api_token = match config.telegram.api_token {
        Some(ref token) => token,
        None => return Ok(()),
    }.as_str();
    let chat_id = match config.telegram.chat_id {
        Some(ref id) => id,
        None => return Ok(()),
    }.as_str();

    if api_token == "" || chat_id == "" {
        return Ok(());
    }

    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("parse_mode", "MarkdownV2");
    map.insert("chat_id", chat_id);

    let text = format!("New vulnerability found:\n```{:#?}```", data);
    map.insert("text", text.as_str());

    client.post(format!(
        "https://api.telegram.org/bot{}/sendMessage", 
        api_token
    ))
        .json(&map)
        .send().await?;
    
    Ok(())
}
