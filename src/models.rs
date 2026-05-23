use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) const MAX_CACHED_TOKEN_EVENTS: usize = 20;
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct TokenUsage {
    pub(crate) input_tokens: u64,
    pub(crate) cached_input_tokens: u64,
    pub(crate) output_tokens: u64,
    pub(crate) reasoning_output_tokens: u64,
    pub(crate) total_tokens: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct TokenEvent {
    pub(crate) timestamp: String,
    pub(crate) total: TokenUsage,
    pub(crate) last: TokenUsage,
    pub(crate) context_window: Option<u64>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct GoalUsage {
    pub(crate) objective: Option<String>,
    pub(crate) status: Option<String>,
    pub(crate) tokens_used: Option<u64>,
    pub(crate) time_used_seconds: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Session {
    pub(crate) id: String,
    pub(crate) timestamp: String,
    pub(crate) path: PathBuf,
    pub(crate) cwd: Option<String>,
    pub(crate) model: Option<String>,
    pub(crate) model_provider: Option<String>,
    pub(crate) first_user_message: Option<String>,
    pub(crate) final_assistant_message: Option<String>,
    pub(crate) token_events: Vec<TokenEvent>,
    pub(crate) goal: GoalUsage,
    pub(crate) web_search_calls: u64,
    pub(crate) line_count: usize,
    pub(crate) parse_errors: Vec<String>,
    #[serde(default, skip)]
    pub(crate) search_messages: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) cached_final_usage: Option<TokenUsage>,
    #[serde(default, skip_serializing_if = "is_zero_u64")]
    pub(crate) max_request_input_tokens: u64,
    #[serde(default, skip_serializing_if = "is_zero_usize")]
    pub(crate) token_event_count: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FileFingerprint {
    pub(crate) size: u64,
    pub(crate) modified_unix_nanos: u64,
    pub(crate) content_hash: String,
    pub(crate) leaf_hash: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct FileMetadataParts {
    pub(crate) size: u64,
    pub(crate) modified_unix_nanos: u64,
}

#[derive(Clone, Debug)]
pub(crate) struct ParsedSessionFile {
    pub(crate) session: Session,
    pub(crate) fingerprint: FileFingerprint,
}
impl TokenUsage {
    pub(crate) fn from_value(value: &Value) -> Self {
        Self {
            input_tokens: json_u64(value.get("input_tokens")).unwrap_or_default(),
            cached_input_tokens: json_u64(value.get("cached_input_tokens"))
                .or_else(|| json_u64(value.get("cache_read_input_tokens")))
                .unwrap_or_default(),
            output_tokens: json_u64(value.get("output_tokens")).unwrap_or_default(),
            reasoning_output_tokens: json_u64(value.get("reasoning_output_tokens"))
                .or_else(|| json_u64(value.get("reasoning_tokens")))
                .unwrap_or_default(),
            total_tokens: json_u64(value.get("total_tokens")).unwrap_or_default(),
        }
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.input_tokens == 0
            && self.cached_input_tokens == 0
            && self.output_tokens == 0
            && self.reasoning_output_tokens == 0
            && self.total_tokens == 0
    }

    pub(crate) fn normalize_total(mut self) -> Self {
        self.cached_input_tokens = self.cached_input_tokens.min(self.input_tokens);
        if self.total_tokens == 0 {
            self.total_tokens =
                self.input_tokens + self.output_tokens + self.reasoning_output_tokens;
        }
        self
    }

    pub(crate) fn saturating_sub(&self, previous: Option<&TokenUsage>) -> Self {
        let previous = previous.cloned().unwrap_or_default();
        Self {
            input_tokens: self.input_tokens.saturating_sub(previous.input_tokens),
            cached_input_tokens: self
                .cached_input_tokens
                .saturating_sub(previous.cached_input_tokens),
            output_tokens: self.output_tokens.saturating_sub(previous.output_tokens),
            reasoning_output_tokens: self
                .reasoning_output_tokens
                .saturating_sub(previous.reasoning_output_tokens),
            total_tokens: self.total_tokens.saturating_sub(previous.total_tokens),
        }
    }

    pub(crate) fn saturating_add(&self, other: &TokenUsage) -> Self {
        Self {
            input_tokens: self.input_tokens.saturating_add(other.input_tokens),
            cached_input_tokens: self
                .cached_input_tokens
                .saturating_add(other.cached_input_tokens),
            output_tokens: self.output_tokens.saturating_add(other.output_tokens),
            reasoning_output_tokens: self
                .reasoning_output_tokens
                .saturating_add(other.reasoning_output_tokens),
            total_tokens: self.total_tokens.saturating_add(other.total_tokens),
        }
        .normalize_total()
    }
}
impl Session {
    pub(crate) fn final_usage(&self) -> Option<&TokenUsage> {
        self.token_events
            .last()
            .map(|event| &event.total)
            .or(self.cached_final_usage.as_ref())
    }

    pub(crate) fn max_request_input(&self) -> u64 {
        self.token_events
            .iter()
            .map(|event| event.last.input_tokens)
            .max()
            .unwrap_or_default()
            .max(self.max_request_input_tokens)
    }

    pub(crate) fn token_event_count(&self) -> usize {
        self.token_event_count.max(self.token_events.len())
    }

    pub(crate) fn token_events_are_truncated(&self) -> bool {
        self.token_event_count() > self.token_events.len()
    }

    pub(crate) fn compact_for_cache(&mut self) {
        self.cached_final_usage = self.final_usage().cloned();
        self.max_request_input_tokens = self.max_request_input();
        self.token_event_count = self.token_events.len();
        if self.token_events.len() > MAX_CACHED_TOKEN_EVENTS {
            let drop_count = self.token_events.len() - MAX_CACHED_TOKEN_EVENTS;
            self.token_events.drain(0..drop_count);
        }
    }
}
pub(crate) fn json_u64(value: Option<&Value>) -> Option<u64> {
    match value? {
        Value::Number(number) => number.as_u64(),
        Value::String(text) => text.trim().parse::<u64>().ok(),
        _ => None,
    }
}
pub(crate) fn is_zero_u64(value: &u64) -> bool {
    *value == 0
}

pub(crate) fn is_zero_usize(value: &usize) -> bool {
    *value == 0
}
