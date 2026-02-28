pub const SITE_NAME: &str = "Rust-DD Blog - Tech Insights & Consulting";
pub const SITE_DESCRIPTION: &str = "Explore open-source Rust projects, learn innovative techniques, and connect with a passionate community. Get expert Rust development and consulting services.";
pub const SITE_URL: &str = "https://rust-dd.com";
pub const DEFAULT_OG_IMAGE: &str = "https://static.rust-dd.com/rust-dd_custom_bg.png";
pub const X_HANDLE: &str = "@rust_dd";

pub fn absolute_url(path: &str) -> String {
    let normalized = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    };

    format!("{SITE_URL}{normalized}")
}
