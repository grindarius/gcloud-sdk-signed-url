use std::{
    collections::{BTreeMap, HashMap},
    sync::LazyLock,
};

use regex::Regex;

static SPACE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(" +").unwrap());
static TAB_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("[\\t]+").unwrap());

pub fn sanitize_headers(headers: HashMap<String, String>) -> Vec<(String, String)> {
    let mut sanitized_headers: BTreeMap<String, String> = BTreeMap::new();

    for (h, v) in headers {
        let sanitized_header = h.trim().to_lowercase();
        let trimmed_value = v.trim();
        let space_concatenated_value = SPACE_REGEX.replace_all(trimmed_value, " ");
        let sanitized_value = TAB_REGEX.replace_all(&space_concatenated_value, "\t");

        let _ = sanitized_headers.insert(sanitized_header, sanitized_value.to_string());
    }

    let mut sanitized_headers_string: Vec<(String, String)> = Vec::new();
    for (h, v) in sanitized_headers {
        sanitized_headers_string.push((h, v));
    }

    sanitized_headers_string
}
