#![allow(dead_code)]

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_parse_i32_vec() {
        assert_eq!(parse_i32_vec("1, 2, 3, 4", &[0]), vec![1, 2, 3, 4]);
        assert_eq!(parse_i32_vec(" 10 , -5 , 20 ", &[0]), vec![10, -5, 20]);
        assert_eq!(parse_i32_vec("invalid, abc", &[1, 2]), vec![1, 2]);
        assert_eq!(parse_i32_vec("", &[5]), vec![5]);
    }

    #[test]
    fn test_parse_string_vec() {
        assert_eq!(
            parse_string_vec("eat, tea, tan", &["default"]),
            vec!["eat", "tea", "tan"]
        );
        assert_eq!(parse_string_vec("", &["a", "b"]), vec!["a", "b"]);
    }

    #[test]
    fn test_parse_tree_nodes() {
        assert_eq!(
            parse_tree_nodes("1, 2, null, 3", &[]),
            vec![Some(1), Some(2), None, Some(3)]
        );
        assert_eq!(
            parse_tree_nodes("NULL, 4, Null", &[]),
            vec![None, Some(4), None]
        );
    }

    #[test]
    fn test_format_python_set() {
        let mut set = BTreeSet::new();
        assert_eq!(format_python_set(&set), "set()");
        set.insert(1);
        set.insert(2);
        assert_eq!(format_python_set(&set), "{1, 2}");
    }
}
