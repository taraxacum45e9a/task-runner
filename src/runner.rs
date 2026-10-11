//! Runs the user's command under `/usr/bin/time`, capturing its output.

use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;

use chrono::{DateTime, Local};

use crate::error::Result;
use crate::sys;

pub struct Run {
    pub success: bool,
    pub started: DateTime<Local>,
    pub finished: DateTime<Local>,
    pub build_log: PathBuf,
    pub time_log: PathBuf,
}

pub fn run(logdir: &Path, workdir: &Path, args: &[String]) -> Result<Run> {
    let started = Local::now();
    let stamp = started.format("%Y%m%d-%H%M%S");
    let time_log = logdir.join(format!("time.{stamp}.log"));
    let build_log = logdir.join(format!("build.{stamp}.log"));

    let out = File::create(&build_log)?;
    let err = out.try_clone()?;

    if let Err(e) = sys::set_priority(-15) {
        eprintln!("warning: setpriority(-15) failed: {e}");
    }

    let status = Command::new("/usr/bin/time")
        .arg("--verbose")
        .arg("-o")
        .arg(&time_log)
        .args(args)
        .stdout(out)
        .stderr(err)
        .current_dir(workdir)
        .status()?;

    Ok(Run {
        success: status.success(),
        started,
        finished: Local::now(),
        build_log,
        time_log,
    })
}
