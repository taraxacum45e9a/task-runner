mod auth;
mod cli;
mod error;
mod notify;
mod pastebin;
mod runner;
mod sys;
mod telegram;

use std::process::ExitCode;

use clap::Parser;

use cli::Cli;
use error::{Error, Result};

fn run() -> Result<()> {
    let cli = Cli::parse();

    let run = runner::run(&cli.logdir, &cli.workdir, &cli.args)?;

    notify::send(&notify::Report {
        success: run.success,
        started: run.started,
        finished: run.finished,
        workdir: &cli.workdir,
        build_log: &run.build_log,
        time_log: &run.time_log,
    })?;
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            if let Error::Http { message, body } = &e {
                sys::syslog_error(message);
                sys::syslog_error(body);
            }
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
