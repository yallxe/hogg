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
    // send telegram message through bot and reqwest
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("parse_mode", "MarkdownV2");
    map.insert("chat_id", config.telegram.chat_id.as_str());

    let text = format!("New vulnerability found:\n```{:#?}```", data);
    map.insert("text", text.as_str());

    client.post(format!(
        "https://api.telegram.org/bot{}/sendMessage", 
        config.telegram.api_token
    ))
        .json(&map)
        .send().await?;
    
    Ok(())
}
