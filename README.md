# task-runner

Run a command, record its output and resource usage, and get the result on
Telegram when it finishes.

For each run, `RUN`:

1. runs the command under GNU `time --verbose`, writing its stdout/stderr to
   `build.<timestamp>.log` and the `time` report to `time.<timestamp>.log`;
2. uploads the tail of the build log to Pastebin (unlisted, expires in a week);
3. sends a Telegram message with the outcome, timings, working directory, the
   paste link and the `time` report;
4. if the command failed, asks an LLM what went wrong and includes the answer.

## Requirements

- Linux with GNU `time` at `/usr/bin/time`
- A syslog socket at `/dev/log` (HTTP errors are logged there)
- Rust (edition 2024) to build
- A Pastebin account and API key, a Telegram bot token and chat ID, and,
  optionally, an OpenAI-compatible LLM endpoint

## Build

Credentials and LLM settings are compiled into the binary from environment
variables. The build fails if any of them is unset.

```sh
cp secrets.env.template secrets.env   # then fill in the values
set -a; source secrets.env; set +a
cargo build --release
```

The binary is `target/release/RUN`. Anyone who can read it can extract
the embedded credentials, so keep it private.

### Configuration

| Variable                 | Purpose                                              |
| ------------------------ | ---------------------------------------------------- |
| `PASTEBIN_DEV_KEY`       | Pastebin API developer key                           |
| `PASTEBIN_USER_NAME`     | Pastebin account name                                |
| `PASTEBIN_USER_PASSWORD` | Pastebin account password                            |
| `TELEGRAM_TOKEN`         | Telegram bot token                                   |
| `TELEGRAM_CHAT_ID`       | Destination chat (integer)                           |
| `OPENAI_BASE_URL`        | LLM endpoint base URL; empty disables summaries      |
| `OPENAI_API_KEY`         | LLM API key                                          |
| `OPENAI_MODEL`           | Model ID, e.g. `qwen3-coder-next`                    |

### Choosing an LLM provider

Any OpenAI-compatible chat-completions API works, and the provider is selected
purely by `OPENAI_BASE_URL`. Include the version prefix; `/chat/completions` is
appended to it:

| Provider          | `OPENAI_BASE_URL`            |
| ----------------- | ---------------------------- |
| SoCLaaS (NUS SoC) | `https://<gateway-host>/v1`  |
| OpenAI            | `https://api.openai.com/v1`  |

Set `OPENAI_API_KEY` and `OPENAI_MODEL` to match the provider.

## Usage

```
RUN [-l <LOGDIR>] [-w <WORKDIR>] <COMMAND> [ARGS...]
```

| Option              | Default | Meaning                              |
| ------------------- | ------- | ------------------------------------ |
| `-l`, `--logdir`    | `.`     | Where the `time.*` and `build.*` logs go |
| `-w`, `--workdir`   | `.`     | Working directory of the command     |

Everything after the first positional argument belongs to the command, so
options of the command itself need no `--`:

```sh
RUN -w ~/src/project -l ~/logs make -j8 all
```

Notes:

- The tool exits `0` once the notification is sent, whatever the command's own
  exit status. It exits non-zero only if running or notifying failed.
- It tries to raise its priority to niceness `-15` (the command inherits it).
  Without privileges this prints a warning and the run continues.
- Pastebin gets at most the last 16 KiB of the build log; the LLM sees the last
  32 KiB. Failures to upload or summarise are reported in the message and never
  block it.

## Development

```sh
cargo test
cargo clippy
```

Source layout:

| Module        | Responsibility                                      |
| ------------- | --------------------------------------------------- |
| `main.rs`     | Entry point and error handling                      |
| `cli.rs`      | Argument parsing                                    |
| `auth.rs`     | Build-time credentials and LLM settings             |
| `runner.rs`   | Runs the command under `time`                       |
| `notify.rs`   | Builds and sends the notification                   |
| `pastebin.rs` | Pastebin upload                                     |
| `telegram.rs` | Telegram client                                     |
| `llm.rs`      | OpenAI-compatible failure summaries                 |
| `logs.rs`     | Log tail helper                                     |
| `sys.rs`      | Hostname, syslog and priority (libc)                |
| `error.rs`    | Error type                                          |
