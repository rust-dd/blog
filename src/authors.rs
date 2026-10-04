pub struct AuthorNote {
    pub name: &'static str,
    pub ident: &'static str,
    pub note: &'static str,
    /// `None` links to Daniel's About page; otherwise the author's profile URL.
    pub href: Option<&'static str>,
}

pub const AUTHORS: &[AuthorNote] = &[
    AuthorNote {
        name: "Daniel Boros",
        ident: "DanielBoros",
        note: "Senior Rust Engineer at Qdrant. Stochastic processes, SIMD, async Rust and Tako.",
        href: None,
    },
    AuthorNote {
        name: "Daniel Zelei",
        ident: "DanielZelei",
        note: "Embedded Rust on the ESP32, deployment, and the Rust 2024 wrap-up.",
        href: Some("https://github.com/zeldan"),
    },
];

pub fn find(name: &str) -> Option<&'static AuthorNote> {
    AUTHORS.iter().find(|author| author.name == name)
}

#[cfg(test)]
mod tests {
    use super::find;

    #[test]
    fn authors_are_found_by_their_display_name() {
        assert_eq!(find("Daniel Boros").map(|author| author.ident), Some("DanielBoros"));
        assert!(find("Someone Else").is_none());
    }
}
