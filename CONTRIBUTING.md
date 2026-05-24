# Contributing

Thanks for helping improve `ccost`.

## Local setup

```bash
git clone https://github.com/peterxcli/ccost.git
cd ccost
cargo test
cargo run -- --help
```

To try the TUI against your own logs:

```bash
cargo run -- --codex
cargo run -- --claude
cargo run -- --sessions /path/to/jsonl/sessions
```

## Good first contributions

Good starter areas include:

- Static README screenshots with sanitized data
- Benchmark scripts and reproducible benchmark numbers
- JSON or CSV export
- Additional log sources such as Cursor, OpenCode, Gemini CLI, or Goose
- Theme and color configuration
- Search-result copy-to-clipboard

## Pull requests

- Keep changes focused.
- Add or update tests when parser, pricing, cache, search, or CLI behavior changes.
- Run `cargo test` before opening a pull request.
- Avoid committing personal session logs, prompts, file paths, stack traces, or secrets.

## Privacy and fixtures

Session logs can contain sensitive prompts, code paths, stack traces, and secrets. Use synthetic or heavily sanitized fixtures in tests and screenshots.
