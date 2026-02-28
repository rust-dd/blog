use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::{
        fa_brands_icons::{FaGithub, FaLinkedin},
        fa_solid_icons::FaRss,
    },
    Icon,
};

#[component]
pub fn Component() -> Element {
    rsx! {
        div { class: "flex flex-row gap-3 items-center",
            a {
                href: "https://github.com/rust-dd/",
                rel: "noopener noreferrer",
                target: "_blank",
                aria_label: "GitHub",
                class: "text-slate-500 transition-colors duration-200 hover:text-slate-800",
                Icon { icon: FaGithub, width: 16, height: 16, fill: "currentColor" }
            }
            a {
                href: "https://x.com/rust_dd",
                rel: "noopener noreferrer",
                target: "_blank",
                aria_label: "X",
                class: "text-slate-500 transition-colors duration-200 hover:text-slate-800",
                svg {
                    view_box: "0 0 512 512",
                    fill: "currentColor",
                    width: "16",
                    height: "16",
                    path { d: "M389.2 48h70.6L305.6 224.2 487 464H345L233.7 318.6 106.5 464H35.8L200.7 275.5 26.8 48h145.6l100.5 132.9L389.2 48zm-24.8 373.8h39.1L154.4 88h-42l252 333.8z" }
                }
            }
            a {
                href: "https://www.linkedin.com/company/rust-dd",
                rel: "noopener noreferrer",
                target: "_blank",
                aria_label: "LinkedIn",
                class: "text-slate-500 transition-colors duration-200 hover:text-slate-800",
                Icon { icon: FaLinkedin, width: 16, height: 16, fill: "currentColor" }
            }
            a {
                href: "/rss.xml",
                rel: "noopener noreferrer",
                target: "_blank",
                aria_label: "RSS",
                class: "text-slate-500 transition-colors duration-200 hover:text-slate-800",
                Icon { icon: FaRss, width: 16, height: 16, fill: "currentColor" }
            }
        }
    }
}
