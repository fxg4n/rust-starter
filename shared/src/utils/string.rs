pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length - 3])
    }
}

pub fn mask_sensitive_data(data: &str) -> String {
    if data.is_empty() {
        return String::new();
    }

    let visible_chars = 4;
    let total_len = data.len();

    if total_len <= visible_chars * 2 {
        return "*".repeat(total_len);
    }

    format!(
        "{}{}{}",
        &data[..visible_chars],
        "*".repeat(total_len - (visible_chars * 2)),
        &data[total_len - visible_chars..]
    )
}
