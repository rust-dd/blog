use std::fmt;

use crate::topics::TOPICS;

/// The home page's `?q=` value. Parsed from the whole query string: the router percent-decodes
/// before it splits on `&`, so a `?:q` argument would cut "rust & zig" at the ampersand.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SearchQuery(pub String);

impl From<&str> for SearchQuery {
    fn from(query: &str) -> Self {
        let value = query
            .strip_prefix("q=")
            .or_else(|| query.split_once("&q=").map(|(_, value)| value))
            .unwrap_or_default();
        Self(value.to_string())
    }
}

impl fmt::Display for SearchQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }
        f.write_str("q=")?;
        for ch in self.0.chars() {
            match ch {
                '%' => f.write_str("%25")?,
                '&' => f.write_str("%26")?,
                _ => write!(f, "{ch}")?,
            }
        }
        Ok(())
    }
}

/// Case-insensitive substring match on a post's title or topic; a blank query matches everything.
pub fn matches(title: &str, topic: Option<&str>, query: &str) -> bool {
    let query = query.trim().to_lowercase();
    // A module name is a filter, not a word: "ai" must not pick up "Explained".
    if TOPICS.iter().any(|known| known.name == query) {
        return topic == Some(query.as_str());
    }
    query.is_empty()
        || title.to_lowercase().contains(&query)
        || topic.is_some_and(|topic| topic.to_lowercase().contains(&query))
}

#[cfg(test)]
mod query_tests {
    use super::SearchQuery;

    #[test]
    fn display_escapes_what_the_router_decodes_before_splitting() {
        assert_eq!(SearchQuery("rust & 100%".into()).to_string(), "q=rust %26 100%25");
    }

    #[test]
    fn empty_query_writes_nothing() {
        assert_eq!(SearchQuery::default().to_string(), "");
    }

    #[test]
    fn parsing_takes_the_q_value_from_a_decoded_query_string() {
        assert_eq!(SearchQuery::from("q=rust & 100%"), SearchQuery("rust & 100%".into()));
        assert_eq!(SearchQuery::from("utm_source=x&q=tako"), SearchQuery("tako".into()));
        assert_eq!(SearchQuery::from("utm_source=x"), SearchQuery::default());
        assert_eq!(SearchQuery::from(""), SearchQuery::default());
    }
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
    fn module_names_match_only_their_own_posts() {
        assert!(!matches("Async Rust Explained - Part 1", Some("async"), "ai"));
        assert!(!matches(
            "Deep Learning the Volatility Surface: An AI-Enhanced Calibration",
            Some("quant"),
            "AI"
        ));
        assert!(matches("iTransformer implementation in pure Rust", Some("ai"), " ai "));
    }

    #[test]
    fn symbols_and_non_ascii_queries_match_literally() {
        assert!(matches("Rust vs Zig vs the father C", None, "zig vs"));
        assert!(matches("Eötvös notes", None, "EÖTVÖS"));
        assert!(!matches("Rust & Zig", None, "c++"));
    }
}
