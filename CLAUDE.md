# task-runner

Rust CLI that runs a command under `/usr/bin/time`, logs its output, and
reports the result to Telegram (build log uploaded to Pastebin). Failed runs
get an LLM-written summary from any OpenAI-compatible endpoint, chosen by
`OPENAI_BASE_URL` (plus `OPENAI_API_KEY`, `OPENAI_MODEL`).

## Build

Credentials are compiled in from environment variables; see
`secrets.env.template`.

```sh
set -a; source secrets.env; set +a
cargo build --release
```

## Commit messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
specification:

```
<type>[optional scope][!]: <description>

[optional body]

[optional footer(s)]
```

- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`,
  `ci`, `chore`, `revert`.
- Scope is optional and names the affected module, e.g. `feat(pastebin): ...`.
- Description is imperative, lowercase, with no trailing period.
- Mark breaking changes with `!` after the type/scope and/or a
  `BREAKING CHANGE:` footer.
