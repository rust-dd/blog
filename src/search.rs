/// Case-insensitive substring match on a post's title or topic; a blank query matches everything.
pub fn matches(title: &str, topic: Option<&str>, query: &str) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty()
        || title.to_lowercase().contains(&query)
        || topic.is_some_and(|topic| topic.to_lowercase().contains(&query))
}

#[cfg(test)]
mod tests {
    use super::matches;

    #[test]
    fn blank_query_matches_everything() {
        assert!(matches("Inline Assembly in Rust", None, "   "));
    }

    #[test]
    fn title_match_ignores_case_and_padding() {
        assert!(matches("Happy SIMD in Rust", Some("low_level"), "  simd "));
    }

    #[test]
    fn topic_name_matches_posts_without_it_in_the_title() {
        assert!(matches("stochastic-rs v1 stable", Some("quant"), "quant"));
        assert!(!matches("stochastic-rs v1 stable", None, "quant"));
    }

    #[test]
    fn symbols_and_non_ascii_queries_match_literally() {
        assert!(matches("Rust vs Zig vs the father C", None, "zig vs"));
        assert!(matches("Eötvös notes", None, "EÖTVÖS"));
        assert!(!matches("Rust & Zig", None, "c++"));
    }
}
