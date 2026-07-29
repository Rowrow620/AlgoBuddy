// ── Shared Input Parsing & Validation Utilities ──

/// Parses a comma-separated string into a Vec<i32>. If empty or invalid, returns fallback.
pub fn parse_i32_vec(input: &str, default: &[i32]) -> Vec<i32> {
    let parsed: Vec<i32> = input
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    if parsed.is_empty() {
        default.to_vec()
    } else {
        parsed
    }
}

/// Parses a comma-separated string into a Vec<String>. If empty, returns fallback.
pub fn parse_string_vec(input: &str, default: &[&str]) -> Vec<String> {
    let parsed: Vec<String> = input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if parsed.is_empty() {
        default.iter().map(|&s| s.to_string()).collect()
    } else {
        parsed
    }
}

/// Parses a comma-separated string of numbers or "null" into a binary tree node array.
pub fn parse_tree_nodes(input: &str, default: &[Option<i32>]) -> Vec<Option<i32>> {
    let parsed: Vec<Option<i32>> = input
        .split(',')
        .map(|s| {
            let trimmed = s.trim();
            if trimmed.eq_ignore_ascii_case("null") || trimmed.is_empty() {
                None
            } else {
                trimmed.parse::<i32>().ok()
            }
        })
        .collect();
    if parsed.is_empty() {
        default.to_vec()
    } else {
        parsed
    }
}

/// Formats a set in authentic Python set syntax: set() when empty, or {1, 2, 3} when populated.
pub fn format_python_set<T: std::fmt::Display>(set: &std::collections::BTreeSet<T>) -> String {
    if set.is_empty() {
        "set()".to_string()
    } else {
        let items: Vec<String> = set.iter().map(|x| x.to_string()).collect();
        format!("{{{}}}", items.join(", "))
    }
}
