use dioxus::prelude::*;

use crate::{
    app::Route,
    seo,
    ssr::api::{hire_us, HireUsRequest},
};

#[component]
pub fn Component() -> Element {
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut subject = use_signal(String::new);
    let mut message = use_signal(String::new);
    let mut sending = use_signal(|| false);
    let mut status = use_signal(|| Option::<String>::None);
    let mut open_faq = use_signal(|| Option::<usize>::None);

    let faqs = [
        (
            "Why choose Rust for your next project?",
            "Rust gives high performance with strong memory safety, so teams can ship reliable systems with fewer production failures.",
        ),
        (
            "What Rust consulting services do you provide?",
            "We cover architecture design, code audits, migrations, performance tuning, and end-to-end delivery of Rust services.",
        ),
        (
            "How quickly can we start?",
            "After a short technical discovery call we can define scope, risks, and an execution plan in a few days.",
        ),
        (
            "Can you help existing teams upskill in Rust?",
            "Yes. We provide focused mentoring and review-driven training tailored to your codebase and engineering goals.",
        ),
    ];

    let title = "Expert Rust Consulting Services | High-Performance Rust Development";
    let description =
        "Professional Rust consulting services specializing in high-performance, reliable, and scalable Rust development.";
    let canonical = seo::absolute_url("/hireus");

    rsx! {
        document::Title { "{title}" }
        document::Meta { name: "description", content: "{description}" }
        document::Meta { name: "robots", content: "index, follow" }
        document::Meta { name: "googlebot", content: "index, follow" }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:title", content: "{title}" }
        document::Meta { property: "og:description", content: "{description}" }
        document::Meta { property: "og:url", content: "{canonical}" }
        document::Meta { property: "og:image", content: seo::DEFAULT_OG_IMAGE }
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: "{title}" }
        document::Meta { name: "twitter:description", content: "{description}" }
        document::Meta { name: "twitter:url", content: "{canonical}" }
        document::Meta { name: "twitter:image", content: seo::DEFAULT_OG_IMAGE }
        document::Link { rel: "canonical", href: "{canonical}" }

        div { class: "w-full space-y-6 font-mono",
            // Hero
            section { class: "py-4",
                p { class: "text-xs text-slate-400", "// consulting" }
                h1 { class: "mt-2 text-3xl font-semibold leading-tight text-slate-900 sm:text-4xl md:text-5xl",
                    "Rust Consulting For Production Teams"
                }
                p { class: "mt-3 max-w-2xl text-sm leading-relaxed text-slate-600",
                    "We help teams ship reliable Rust systems, improve performance bottlenecks, and de-risk architecture decisions."
                }
                div { class: "mt-4 flex flex-wrap gap-3",
                    Link {
                        to: Route::References {},
                        class: "inline-flex items-center rounded border border-slate-200 bg-white px-3 py-1.5 text-xs text-slate-600 transition-colors duration-200 hover:border-slate-400",
                        "see references"
                    }
                    a {
                        href: "#contact",
                        class: "inline-flex items-center rounded border border-slate-200 bg-white px-3 py-1.5 text-xs text-slate-600 transition-colors duration-200 hover:border-slate-400",
                        "contact us"
                    }
                }
            }

            // Services
            section { class: "grid gap-4 md:grid-cols-2",
                ServiceCard {
                    title: "Architecture & Delivery",
                    text: "System design, migration strategy, and hands-on implementation for critical Rust services."
                }
                ServiceCard {
                    title: "Performance & Reliability",
                    text: "Profiling, observability, and targeted optimizations for throughput, latency, and runtime stability."
                }
            }

            // Team
            section { class: "rounded-lg border border-slate-200 bg-white p-4 sm:p-6",
                p { class: "text-xs text-slate-400", "// team" }
                h2 { class: "mt-2 text-2xl font-semibold text-slate-900 sm:text-3xl", "Senior Rust Experts" }
                div { class: "mt-4 grid gap-4 md:grid-cols-2",
                    ExpertCard {
                        name: "Daniel Zelei",
                        role: "Senior Rust Consultant",
                        image: "https://static.rust-dd.com/zelei.webp",
                        linkedin: "https://www.linkedin.com/in/danielzelei/"
                    }
                    ExpertCard {
                        name: "Daniel Boros",
                        role: "Senior Rust Consultant",
                        image: "https://static.rust-dd.com/boros.webp",
                        linkedin: "https://www.linkedin.com/in/daniel-boros-b86a5373/"
                    }
                }
            }

            // Contact + FAQ
            section { class: "grid gap-6 lg:grid-cols-2",
                div { id: "contact", class: "rounded-lg border border-slate-200 bg-white p-4 sm:p-6",
                    p { class: "text-xs text-slate-400", "// contact" }
                    h2 { class: "mt-2 text-2xl font-semibold text-slate-900 sm:text-3xl", "Tell us what you are building" }

                    form {
                        class: "mt-5 space-y-4",
                        onsubmit: move |event| {
                            event.prevent_default();
                            if *sending.read() {
                                return;
                            }

                            sending.set(true);
                            status.set(None);
                            let payload = HireUsRequest {
                                name: name(),
                                email: email(),
                                subject: subject(),
                                message: message(),
                            };

                            spawn(async move {
                                let result = hire_us(payload).await;
                                sending.set(false);

                                match result {
                                    Ok(_) => {
                                        name.set(String::new());
                                        email.set(String::new());
                                        subject.set(String::new());
                                        message.set(String::new());
                                        status.set(Some(
                                            "Message sent successfully. We will get back shortly.".to_string(),
                                        ));
                                    }
                                    Err(err) => status.set(Some(format!("Failed to send message: {err}"))),
                                }
                            });
                        },

                        div { class: "grid gap-4 sm:grid-cols-2",
                            input {
                                placeholder: "Your Name",
                                r#type: "text",
                                required: true,
                                value: "{name()}",
                                oninput: move |event| name.set(event.value()),
                                class: "w-full rounded border border-slate-200 bg-white px-3 py-2.5 text-sm text-slate-900 placeholder:text-slate-400 focus:border-slate-400 focus:outline-none"
                            }
                            input {
                                placeholder: "Your Email",
                                r#type: "email",
                                required: true,
                                value: "{email()}",
                                oninput: move |event| email.set(event.value()),
                                class: "w-full rounded border border-slate-200 bg-white px-3 py-2.5 text-sm text-slate-900 placeholder:text-slate-400 focus:border-slate-400 focus:outline-none"
                            }
                        }

                        input {
                            placeholder: "Subject",
                            r#type: "text",
                            required: true,
                            value: "{subject()}",
                            oninput: move |event| subject.set(event.value()),
                            class: "w-full rounded border border-slate-200 bg-white px-3 py-2.5 text-sm text-slate-900 placeholder:text-slate-400 focus:border-slate-400 focus:outline-none"
                        }

                        textarea {
                            placeholder: "Your Message",
                            rows: 6,
                            required: true,
                            value: "{message()}",
                            oninput: move |event| message.set(event.value()),
                            class: "w-full rounded border border-slate-200 bg-white px-3 py-2.5 text-sm text-slate-900 placeholder:text-slate-400 focus:border-slate-400 focus:outline-none"
                        }

                        button {
                            r#type: "submit",
                            disabled: *sending.read(),
                            class: "inline-flex w-full items-center justify-center rounded bg-slate-700 px-4 py-2.5 text-sm font-semibold text-white transition-colors duration-200 hover:bg-slate-800 disabled:cursor-not-allowed disabled:opacity-70",
                            if *sending.read() {
                                "Sending..."
                            } else {
                                "Send Message"
                            }
                        }

                        if let Some(current_status) = status() {
                            p { class: "text-sm text-slate-600", "{current_status}" }
                        }
                    }
                }

                div { class: "rounded-lg border border-slate-200 bg-white p-4 sm:p-6",
                    p { class: "text-xs text-slate-400", "// faq" }
                    h2 { class: "mt-2 text-2xl font-semibold text-slate-900 sm:text-3xl", "Common Questions" }

                    div { class: "mt-5 space-y-2",
                        for (index, faq) in faqs.iter().enumerate() {
                            article { class: "rounded border border-slate-200 bg-slate-50/50",
                                button {
                                    class: "flex w-full items-center justify-between gap-4 px-4 py-3 text-left",
                                    onclick: move |_| {
                                        if open_faq() == Some(index) {
                                            open_faq.set(None);
                                        } else {
                                            open_faq.set(Some(index));
                                        }
                                    },
                                    h3 { class: "text-sm font-semibold text-slate-900", "{faq.0}" }
                                    span { class: "text-xs text-slate-400", if open_faq() == Some(index) { "-" } else { "+" } }
                                }

                                if open_faq() == Some(index) {
                                    div { class: "px-4 pb-4",
                                        p { class: "text-sm leading-relaxed text-slate-600", "{faq.1}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ServiceCard(title: &'static str, text: &'static str) -> Element {
    rsx! {
        article { class: "rounded-lg border border-slate-200 bg-white p-4 sm:p-5",
            p { class: "text-xs text-slate-400", "// service" }
            h3 { class: "mt-2 text-xl font-semibold text-slate-900", "{title}" }
            p { class: "mt-2 text-sm leading-relaxed text-slate-600", "{text}" }
        }
    }
}

#[component]
fn ExpertCard(name: &'static str, role: &'static str, image: &'static str, linkedin: &'static str) -> Element {
    rsx! {
        article { class: "flex items-center gap-4 rounded-lg border border-slate-200 bg-slate-50/50 p-3",
            img {
                src: "{image}",
                alt: "{name}",
                width: 72,
                height: 72,
                class: "h-[72px] w-[72px] rounded-full border border-slate-200 object-cover"
            }
            div { class: "min-w-0",
                h3 { class: "truncate text-lg font-semibold text-slate-900", "{name}" }
                p { class: "mt-1 text-sm text-slate-600", "{role}" }
                a {
                    href: "{linkedin}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "mt-2 inline-flex text-xs text-slate-500 hover:text-slate-700",
                    "LinkedIn >"
                }
            }
        }
    }
}
