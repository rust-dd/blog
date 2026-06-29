use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_solid_icons::{FaMoon, FaSun},
    Icon,
};

#[component]
pub fn Component() -> Element {
    let mut theme = use_signal(|| "dark".to_string());

    use_effect(move || {
        spawn(async move {
            let eval = document::eval(
                "return document.documentElement.getAttribute('data-theme') || 'dark';",
            );
            if let Ok(value) = eval.await {
                if let Some(current) = value.as_str() {
                    theme.set(current.to_string());
                }
            }
        });
    });

    let toggle = move |_| {
        spawn(async move {
            let eval = document::eval(
                "var c=document.documentElement.getAttribute('data-theme')||'dark';\
                 var n=c==='dark'?'light':'dark';\
                 document.documentElement.setAttribute('data-theme',n);\
                 try{localStorage.setItem('theme',n)}catch(e){}\
                 return n;",
            );
            if let Ok(value) = eval.await {
                if let Some(next) = value.as_str() {
                    theme.set(next.to_string());
                }
            }
        });
    };

    let is_dark = theme() == "dark";

    rsx! {
        button {
            r#type: "button",
            aria_label: "Toggle color theme",
            title: "Toggle theme",
            onclick: toggle,
            class: "inline-flex h-8 w-8 items-center justify-center rounded-md border border-border text-muted transition-colors duration-200 hover:border-accent hover:text-accent",
            if is_dark {
                Icon { icon: FaSun, width: 15, height: 15, fill: "currentColor" }
            } else {
                Icon { icon: FaMoon, width: 15, height: 15, fill: "currentColor" }
            }
        }
    }
}
