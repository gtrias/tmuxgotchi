/// Map raw model identifiers to human-friendly display names.
pub fn display_name(model_id: &str) -> &str {
    // Handle both raw IDs and already-formatted names
    match model_id {
        // Raw API model IDs
        "claude-opus-4-6" => "Opus 4.6",
        "claude-sonnet-4-6" => "Sonnet 4.6",
        "claude-sonnet-4-5-20250514" => "Sonnet 4.5",
        "claude-haiku-4-5-20251001" => "Haiku 4.5",
        "claude-opus-4-20250514" => "Opus 4",
        "claude-sonnet-4-20250514" => "Sonnet 4",

        // Already formatted (from status bar)
        s if s.starts_with("Opus") => s,
        s if s.starts_with("Sonnet") => s,
        s if s.starts_with("Haiku") => s,

        // Pass through unknown
        _ => model_id,
    }
}

/// Context window size for a given model.
pub fn context_window(model_id: &str) -> u64 {
    match model_id {
        "claude-opus-4-6" | "Opus 4.6" => 1_000_000,
        _ => 200_000,
    }
}
