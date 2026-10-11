# task-runner

Rust CLI that runs a command, logs its output and reports the result to
Telegram, with an LLM summary for failures. Purpose, build steps,
configuration variables, usage and module layout are in @README.md; keep that
file up to date when behaviour or configuration changes.

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
