//! Failure summaries from any OpenAI-compatible chat-completions endpoint.
//! The provider is selected solely by `auth::OPENAI_BASE_URL`.

use std::time::Duration;

use serde_json::{Value, json};

use crate::auth;
use crate::error::{Error, Result};

const TIMEOUT: Duration = Duration::from_secs(60);

const SYSTEM_PROMPT: &str = "You analyse the output of a command that failed. \
In at most five short sentences, explain what went wrong and the most likely \
cause, quoting the key error line if there is one. Plain text only: no markdown, \
no HTML, no preamble.";

/// Whether a provider is configured.
pub fn enabled() -> bool {
    !auth::OPENAI_BASE_URL.is_empty()
}

/// What the model gets to see about a failed run.
pub struct Failure<'a> {
    pub command: &'a [String],
    pub exit: &'a str,
    pub time_log: &'a str,
    pub output_tail: &'a str,
}

fn endpoint(base: &str) -> String {
    format!("{}/chat/completions", base.trim_end_matches('/'))
}

fn prompt(f: &Failure) -> String {
    format!(
        "Command: {}\nResult: {}\n\n--- resource usage (time) ---\n{}\n\n--- output (tail) ---\n{}",
        f.command.join(" "),
        f.exit,
        f.time_log.trim_end(),
        f.output_tail.trim_end(),
    )
}

pub fn summarise(failure: &Failure) -> Result<String> {
    let resp = ureq::AgentBuilder::new()
        .timeout(TIMEOUT)
        .build()
        .post(&endpoint(auth::OPENAI_BASE_URL))
        .set("Authorization", &format!("Bearer {}", auth::OPENAI_API_KEY))
        .send_json(json!({
            "model": auth::OPENAI_MODEL,
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": prompt(failure)},
            ],
        }))?;

    let body: Value = resp.into_json()?;
    body.pointer("/choices/0/message/content")
        .and_then(Value::as_str)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| Error::Other("no completion in LLM response".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_tolerates_trailing_slash() {
        assert_eq!(endpoint("http://h/v1"), "http://h/v1/chat/completions");
        assert_eq!(endpoint("http://h/v1/"), "http://h/v1/chat/completions");
    }

    #[test]
    fn prompt_contains_all_parts() {
        let cmd = vec!["make".to_string(), "-j4".to_string()];
        let p = prompt(&Failure {
            command: &cmd,
            exit: "exit status: 2",
            time_log: "Elapsed: 1s\n",
            output_tail: "error: boom\n",
        });
        for needle in ["make -j4", "exit status: 2", "Elapsed: 1s", "error: boom"] {
            assert!(p.contains(needle), "missing {needle}");
        }
    }
}
