pub struct Topic {
    pub name: &'static str,
    pub description: &'static str,
}

pub const TOPICS: &[Topic] = &[
    Topic {
        name: "async",
        description: "Futures and pinning explained, and what to put behind a lock.",
    },
    Topic {
        name: "web",
        description: "Tako, GraphQL and WebSocket backends, and Rust in the browser through WebAssembly.",
    },
    Topic {
        name: "low_level",
        description: "SIMD, inline assembly, union types, bit tricks and a Rust vs Zig vs C benchmark.",
    },
    Topic {
        name: "quant",
        description: "Stochastic processes, rough volatility and option pricing with stochastic-rs.",
    },
    Topic {
        name: "embedded",
        description: "Rust on the ESP32, from a blinking LED to a DHT11 sensor driver.",
    },
    Topic {
        name: "mobile",
        description: "Rust inside React Native and Flutter apps.",
    },
    Topic {
        name: "ai",
        description: "Retrieval-augmented generation and the iTransformer in pure Rust.",
    },
    Topic {
        name: "ops",
        description: "Deploying Rust to Google Cloud Run.",
    },
    Topic {
        name: "meta",
        description: "News from Rust-DD and a look back at Rust in 2024.",
    },
];

#[cfg(test)]
mod tests {
    use super::TOPICS;

    #[test]
    fn topic_names_are_unique_snake_case_identifiers() {
        let mut names: Vec<&str> = TOPICS.iter().map(|topic| topic.name).collect();
        assert!(names
            .iter()
            .all(|name| !name.is_empty() && name.chars().all(|ch| ch.is_ascii_lowercase() || ch == '_')));
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), TOPICS.len());
    }
}
