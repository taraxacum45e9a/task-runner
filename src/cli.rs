use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Directory for the time and build logs
    #[arg(short, long, default_value_os_t = std::env::current_dir().unwrap_or_default())]
    pub logdir: PathBuf,

    /// Working directory of the command
    #[arg(short, long, default_value_os_t = std::env::current_dir().unwrap_or_default())]
    pub workdir: PathBuf,

    /// Command to run, with its arguments
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
}
