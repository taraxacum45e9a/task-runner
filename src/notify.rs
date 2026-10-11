//! Builds and sends the run-summary notification.

use std::fs;
use std::path::Path;

use chrono::{DateTime, Local};
use serde_json::Value;

use crate::error::{Error, Result};
use crate::{pastebin, sys, telegram};

/// What happened during a run, for reporting.
pub struct Report<'a> {
    pub success: bool,
    pub started: DateTime<Local>,
    pub finished: DateTime<Local>,
    pub workdir: &'a Path,
    pub build_log: &'a Path,
    pub time_log: &'a Path,
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

    match pastebin::upload(r.build_log) {
        Ok(url) => body.push(url),
        Err(Error::Http { message, body: b }) => {
            body.push(format!("⚠️ Error: {message}"));
            body.push(format!("⚠️ Error: {}", b.trim()));
        }
        Err(e) => body.push(format!("⚠️ Error: {e}")),
    }

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
