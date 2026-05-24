use crate::cli::{claude_sessions_dir, codex_sessions_dir, Args};

#[test]
fn no_session_flag_keeps_default_runtime_behavior() {
    let args = Args::parse_from(["ccost"]).unwrap();

    assert_eq!(args.sessions, None);
}

#[test]
fn codex_flag_selects_default_codex_sessions_dir() {
    let args = Args::parse_from(["ccost", "--codex"]).unwrap();

    assert_eq!(args.sessions, Some(codex_sessions_dir()));
}

#[test]
fn claude_flag_selects_default_claude_sessions_dir() {
    let args = Args::parse_from(["ccost", "--claude"]).unwrap();

    assert_eq!(args.sessions, Some(claude_sessions_dir()));
}

#[test]
fn provider_flags_conflict_with_sessions_path() {
    let error = Args::parse_from(["ccost", "--sessions", "/tmp/sessions", "--claude"])
        .unwrap_err()
        .to_string();

    assert!(error.contains("cannot be used with --sessions"));
}

#[test]
fn codex_and_claude_flags_conflict() {
    let error = Args::parse_from(["ccost", "--codex", "--claude"])
        .unwrap_err()
        .to_string();

    assert!(error.contains("cannot be used together"));
}
