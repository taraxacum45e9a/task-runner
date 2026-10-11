use serde_json::{Value, json};

use crate::auth;
use crate::error::{Error, Result};

/// Send an HTML rich message to the configured chat.
pub fn send(html: &str) -> Result<Value> {
    if auth::TELEGRAM_TOKEN.is_empty() {
        return Err(Error::Other("TELEGRAM_BOT_TOKEN must be set".into()));
    }
    if auth::TELEGRAM_CHAT_ID.is_empty() {
        return Err(Error::Other("TELEGRAM_CHAT_ID must be set".into()));
    }
    let chat_id: i64 = auth::TELEGRAM_CHAT_ID
        .parse()
        .map_err(|_| Error::Other("TELEGRAM_CHAT_ID must be an integer".into()))?;

    let resp = ureq::post(&format!(
        "https://api.telegram.org/bot{}/sendRichMessage",
        auth::TELEGRAM_TOKEN
    ))
    .send_json(json!({ "chat_id": chat_id, "rich_message": { "html": html } }))?;
    Ok(resp.into_json()?)
}
