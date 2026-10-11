//! Credentials, injected through environment variables at build time.
//! See `secrets.env.template`.

pub const PASTEBIN_DEV_KEY: &str = env!("PASTEBIN_DEV_KEY", "set PASTEBIN_DEV_KEY at build time");
pub const PASTEBIN_USER_NAME: &str =
    env!("PASTEBIN_USER_NAME", "set PASTEBIN_USER_NAME at build time");
pub const PASTEBIN_USER_PASSWORD: &str = env!(
    "PASTEBIN_USER_PASSWORD",
    "set PASTEBIN_USER_PASSWORD at build time"
);
pub const TELEGRAM_TOKEN: &str = env!("TELEGRAM_TOKEN", "set TELEGRAM_TOKEN at build time");
pub const TELEGRAM_CHAT_ID: &str = env!("TELEGRAM_CHAT_ID", "set TELEGRAM_CHAT_ID at build time");

// OpenAI-compatible chat endpoint used to summarise failed runs. The provider
// is chosen by the base URL (including any `/v1`); leave it empty to disable.
pub const OPENAI_BASE_URL: &str = env!(
    "OPENAI_BASE_URL",
    "set OPENAI_BASE_URL at build time (empty disables summaries)"
);
pub const OPENAI_API_KEY: &str = env!("OPENAI_API_KEY", "set OPENAI_API_KEY at build time");
pub const OPENAI_MODEL: &str = env!("OPENAI_MODEL", "set OPENAI_MODEL at build time");
