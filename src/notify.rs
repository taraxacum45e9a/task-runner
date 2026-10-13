//! Builds and sends the run-summary notification.

use std::fs;
use std::path::Path;

use chrono::{DateTime, Local};
use serde_json::Value;

use crate::error::{Error, Result};
use crate::{llm, logs, pastebin, sys, telegram};

/// Build logs up to this many bytes go straight into the message; larger ones
/// are uploaded to Pastebin.
const INLINE_LOG_LIMIT: u64 = 1024;

/// Bytes of build output / time log shown to the model.
const LLM_OUTPUT_LIMIT: u64 = 32 << 10;
const LLM_TIME_LIMIT: u64 = 8 << 10;

/// What happened during a run, for reporting.
pub struct Report<'a> {
    pub success: bool,
    pub exit: &'a str,
    pub command: &'a [String],
    pub started: DateTime<Local>,
    pub finished: DateTime<Local>,
    pub workdir: &'a Path,
    pub build_log: &'a Path,
    pub time_log: &'a Path,
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// LLM explanation of a failed run, as message lines.
fn failure_summary(r: &Report) -> Vec<String> {
    let input = logs::tail(r.build_log, LLM_OUTPUT_LIMIT).map(|output| {
        let time = logs::tail(r.time_log, LLM_TIME_LIMIT).unwrap_or_default();
        (output, time)
    });
    let result = input
        .map_err(Error::from)
        .and_then(|(output_tail, time_log)| {
            llm::summarise(&llm::Failure {
                command: r.command,
                exit: r.exit,
                time_log: &time_log,
                output_tail: &output_tail,
            })
        });
    match result {
        Ok(text) => vec![
            "<b>🤖 Summary</b>".into(),
            escape_html(&text),
            String::new(),
        ],
        Err(Error::Http { message, body }) => vec![
            format!("⚠️ Summary error: {}", escape_html(&message)),
            format!("⚠️ Summary error: {}", escape_html(body.trim())),
            String::new(),
        ],
        Err(e) => vec![
            format!("⚠️ Summary error: {}", escape_html(&e.to_string())),
            String::new(),
        ],
    }
}

/// Inline rendering of a small build log.
fn inline_log(text: &str) -> String {
    let text = text.trim_end();
    if text.is_empty() {
        "(build log is empty)".to_string()
    } else {
        format!("<pre>{}</pre>", escape_html(text))
    }
}

/// The build log as message lines: inline if small, else a Pastebin link.
fn build_log_section(path: &Path) -> Vec<String> {
    let inline = fs::metadata(path)
        .map(|m| m.len() <= INLINE_LOG_LIMIT)
        .unwrap_or(false);
    if inline {
        return match fs::read(path) {
            Ok(bytes) => vec![inline_log(&String::from_utf8_lossy(&bytes))],
            Err(e) => vec![format!("⚠️ Error: {}", escape_html(&e.to_string()))],
        };
    }
    match pastebin::upload(path) {
        Ok(url) => vec![url],
        Err(Error::Http { message, body }) => vec![
            format!("⚠️ Error: {message}"),
            format!("⚠️ Error: {}", body.trim()),
        ],
        Err(e) => vec![format!("⚠️ Error: {e}")],
    }
}

/// C `ctime()`-style timestamp.
fn ctime(t: &DateTime<Local>) -> String {
    t.format("%a %b %e %H:%M:%S %Y").to_string()
}

/// Collapsible section built from the `time` log: its last line is the
/// summary, the rest is the body.
fn time_log_section(path: &Path) -> Result<Vec<String>> {
    let text = String::from_utf8_lossy(&fs::read(path)?).into_owned();
    let mut raw: Vec<&str> = text.lines().collect();
    let summary = raw
        .pop()
        .ok_or_else(|| Error::Other("pop from empty list".into()))?;
    Ok(vec![
        "<details>".into(),
        format!("<summary>{summary}</summary>"),
        format!("<pre>{}</pre>", raw.join("\n")),
        "</details>".into(),
    ])
}

fn render(r: &Report) -> String {
    let host = sys::hostname();
    let summary = if r.success {
        format!("✅ Command run succeeded on {host}")
    } else {
        format!("❌ Command run failed on {host}")
    };

    let mut body = vec![
        format!("<b>{summary}</b>"),
        "<ul>".to_string(),
        format!("<li>Started at: {}</li>", ctime(&r.started)),
        format!("<li>Finished at: {}</li>", ctime(&r.finished)),
        format!("<li>Working directory: {}</li>", r.workdir.display()),
        format!("<li>Build log: {}</li>", r.build_log.display()),
        "</ul>".to_string(),
    ];

    if !r.success && llm::enabled() {
        body.extend(failure_summary(r));
    }

    body.extend(build_log_section(r.build_log));

    body.push(String::new());

    match time_log_section(r.time_log) {
        Ok(lines) => body.extend(lines),
        Err(e) => body.push(format!("⚠️ Error: {e}")),
    }

    body.join("\n")
}

pub fn send(report: &Report) -> Result<Value> {
    telegram::send(&render(report))
}

#[cfg(test)]
mod tests {
    use super::{escape_html, inline_log};

    #[test]
    fn escapes_html_specials() {
        assert_eq!(escape_html("a<b>&c"), "a&lt;b&gt;&amp;c");
    }

    #[test]
    fn inline_log_is_escaped_and_trimmed() {
        assert_eq!(inline_log("a <b>\n\n"), "<pre>a &lt;b&gt;</pre>");
    }

    #[test]
    fn empty_inline_log_has_placeholder() {
        assert_eq!(inline_log("\n"), "(build log is empty)");
    }
}
