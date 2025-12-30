//! String utilities for policy evaluation

use regex::Regex;

/// Check if string contains substring
pub fn contains(haystack: &str, needle: &str) -> bool {
    haystack.contains(needle)
}

/// Split string by delimiter
pub fn split(text: &str, delimiter: &str) -> Vec<String> {
    text.split(delimiter).map(|s| s.to_string()).collect()
}

/// Format string with placeholders (simple implementation)
pub fn format(template: &str, args: Vec<String>) -> String {
    let mut result = template.to_string();
    for (i, arg) in args.iter().enumerate() {
        let placeholder = format!("{{{}}}", i);
        result = result.replace(&placeholder, arg);
    }
    result
}

pub fn matches_regex(text: &str, pattern: &str) -> bool {
    Regex::new(pattern).map(|re| re.is_match(text)).unwrap_or(false)
}

pub fn replace(text: &str, from: &str, to: &str) -> String {
    text.replace(from, to)
}

pub fn replace_first(text: &str, pattern: &str, to: &str) -> String {
    Regex::new(pattern)
        .ok()
        .and_then(|re| re.replace(text, to).into_owned().into())
        .unwrap_or_else(|| text.to_string())
}

/// Convert string to lowercase
pub fn lowercase(text: &str) -> String {
    text.to_lowercase()
}

/// Convert string to uppercase
pub fn uppercase(text: &str) -> String {
    text.to_uppercase()
}

/// Trim whitespace from string
pub fn trim(text: &str) -> String {
    text.trim().to_string()
}

/// Get string length
pub fn length(text: &str) -> usize {
    text.len()
}

/// Check if string starts with prefix
pub fn starts_with(text: &str, prefix: &str) -> bool {
    text.starts_with(prefix)
}

/// Check if string ends with suffix
pub fn ends_with(text: &str, suffix: &str) -> bool {
    text.ends_with(suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        assert!(contains("hello world", "world"));
        assert!(!contains("hello world", "foo"));
    }

    #[test]
    fn test_split() {
        assert_eq!(split("a,b,c", ","), vec!["a", "b", "c"]);
        assert_eq!(split("single", ","), vec!["single"]);
    }

    #[test]
    fn test_format() {
        assert_eq!(format("Hello {0}!", vec!["World".to_string()]), "Hello World!");
        assert_eq!(format("{0} + {1} = {2}", vec!["1".to_string(), "2".to_string(), "3".to_string()]), "1 + 2 = 3");
    }

    #[test]
    fn test_regex_replace() {
        assert!(matches_regex("hello world", "world"));
        assert!(!matches_regex("hello", "^world$"));
        assert_eq!(replace("a-b-c", "-", ":"), "a:b:c");
        assert_eq!(replace_first("abc123", "\\d+", "#"), "abc#");
    }
}
