# ccost

**Find old Codex and Claude Code sessions in seconds, then see which ones burned the most tokens.**

`ccost` turns local AI coding session logs into a searchable terminal UI with a cost lens. Use it to recover past prompts, inspect expensive runs, and understand which sessions, models, and web-search calls drove your spend.

https://github.com/user-attachments/assets/0869bcde-96be-4d98-9c07-c0586b0ea36a

```bash
brew install --cask peterxcli/tap/ccost
ccost
```

## Why use it?

- Find the session where you fixed a bug, explored an API, or hit a useful error.
- Sort sessions by estimated cost, tokens, time, model, web searches, session id, or first prompt.
- Browse Codex and Claude Code logs without uploading prompts, paths, stack traces, or source context.
- Keep startup fast with a persisted search index that only re-indexes changed files.

## Supported Sources

| Source | Default path |
| --- | --- |
| Codex | `$CODEX_HOME/sessions`, or `~/.codex/sessions` when `CODEX_HOME` is unset |
| Claude Code | `~/.claude/projects` |
| Custom JSONL directory | `ccost --sessions /path/to/jsonl/sessions` |

## Install

```bash
brew install --cask peterxcli/tap/ccost
```

Upgrade:

```bash
brew update
brew upgrade --cask ccost
```

Other install paths are on the roadmap:

- `cargo install ccost`
- `cargo binstall ccost`
- Linux release artifacts
- Shell install script

## Run

Open the default Codex session directory:

```bash
ccost
```

Choose a source explicitly:

```bash
ccost --codex
ccost --claude
```

Use a custom session directory:

```bash
ccost --sessions /path/to/jsonl/sessions
```

## Use it when...

### Find an old fix

You remember asking Codex or Claude Code about `JWT refresh`, but not which project or session it was in.

Press `/`, type:

```txt
JWT refresh
```

### Find expensive sessions

Sort by cost to see which runs used the most estimated spend.

Press:

```txt
s
```

until sort is `cost`.

### Audit web-search-heavy sessions

Sort by web searches to find sessions where search calls contributed to the estimate.

## Privacy

`ccost` is local-first.

- No API key required
- No telemetry
- No network calls
- No uploads
- Session JSONL files remain the source of truth
- Cache files are disposable and stored locally

If cache format drift or corruption is detected, delete the cache folder shown in the TUI. The original session logs are not modified.

## Features

- Full-text search across local session logs.
- Prefix matching, match highlighting, and visible search cursor.
- Cost-aware sorting by total cost, time, tokens, web searches, model, session id, or first prompt.
- Codex and Claude Code defaults, plus custom JSONL session directories.
- Built-in pricing table with local JSON override support.
- Optional `--no-web-cost` mode for token-only estimates.
- Single-writer cache lock with read-only and force-write escape hatches.

## Why it is fast

- Rust terminal UI.
- Persisted full-text index.
- FST term index for compact prefix search.
- Persisted Merkle tree and file watcher, so unchanged sessions are reused and changed session files are re-indexed incrementally.

## Controls

| Key | Action |
| --- | --- |
| `/` | Enter search mode |
| `Enter` | Return to browse mode, or toggle detail while browsing |
| `Up` / `Down` or `j` / `k` | Move selection |
| `Tab` | Switch list/detail focus |
| `s` | Next sort key |
| `S` | Reverse sort direction |
| `r` | Reload |
| `Esc` | Clear search or go back |
| `q` | Quit |

## Options

```bash
ccost [--codex | --claude | --sessions PATH] [--pricing PATH] [--no-web-cost] [--read-only-index] [--force-index]
```

- `--codex`: open the default Codex session directory.
- `--claude`: open the default Claude Code project transcript directory.
- `--sessions PATH`: open a custom Codex or Claude Code session directory containing JSONL files.
- `--pricing PATH`: load a local pricing JSON override.
- `--no-web-cost`: disable web-search call cost in estimates.
- `--read-only-index`: open without writing the persisted search cache.
- `--force-index`: write without the lock. Use only after confirming no other TUI is running.

## Pricing

Cost is an estimate based on the token usage and model data present in local session logs. Built-in pricing includes GPT-5.5, GPT-5.4, Claude Opus/Sonnet/Haiku model families, and web-search defaults. Override the local table with `--pricing pricing.json`:

```json
{
  "web_search_per_1k": 10.0,
  "models": {
    "gpt-5.5": {
      "input_per_m": 5.0,
      "cache_creation_input_per_m": 0.0,
      "cached_input_per_m": 0.5,
      "output_per_m": 30.0,
      "long_context_threshold": 272000,
      "long_context_input_multiplier": 2.0,
      "long_context_output_multiplier": 1.5
    },
    "gpt-5.4": {
      "input_per_m": 2.5,
      "cache_creation_input_per_m": 0.0,
      "cached_input_per_m": 0.25,
      "output_per_m": 15.0,
      "long_context_threshold": 272000,
      "long_context_input_multiplier": 2.0,
      "long_context_output_multiplier": 1.5
    },
    "claude-sonnet-4-5": {
      "input_per_m": 3.0,
      "cache_creation_input_per_m": 3.75,
      "cached_input_per_m": 0.30,
      "output_per_m": 15.0
    }
  }
}
```

For older pricing overrides, `long_context_multiplier` is still accepted and applies to all token classes when the input/output-specific fields are omitted. `cache_creation_input_per_m` is optional and defaults to `0.0`.

## Benchmarks

Benchmark numbers should be measured on a named machine and reproducible dataset before being used as proof. The launch checklist in [docs/github-launch.md](docs/github-launch.md) includes the table to fill in once those numbers are available.

## How is this different?

| Tool | Focus |
| --- | --- |
| `ccusage` | CLI reports for token and cost usage across many agents |
| CodeBurn | Cost observability dashboard across AI coding tools |
| `ccost` | Fast local TUI for searching and browsing Codex and Claude Code sessions, with cost-aware sorting |

## Roadmap

- [ ] Static README screenshot with sanitized data
- [ ] GitHub social preview image
- [ ] `cargo install ccost`
- [ ] Linux release artifacts
- [ ] Demo fixture or `ccost --demo`
- [ ] JSON/CSV export
- [ ] Project-level cost summaries
- [ ] Cursor, OpenCode, Gemini CLI, and Goose log support
- [ ] Configurable themes
- [ ] Benchmarks in CI

## Contributing

Contributions are welcome, especially for new log sources and export formats. See [CONTRIBUTING.md](CONTRIBUTING.md).

If `ccost` helps you recover an old session or understand where tokens went, a star helps show it is worth maintaining.
