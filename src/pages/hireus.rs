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

    let faqs = vec![
        (
            "Why choose Rust for your next project?",
            "Rust ensures performance, safety, and reliability, ideal for system-critical applications, embedded solutions, and high-performance computing.",
        ),
        (
            "What Rust consulting services do we offer?",
            "We offer Rust consulting, architecture design, performance optimization, code audits, training, and custom Rust development solutions.",
        ),
        (
            "How can Rust consulting benefit my business?",
            "Our expert Rust consultants help businesses build faster, safer, and more scalable software, reducing technical debt and operational risk.",
        ),
        (
            "Is Rust suitable for web development?",
            "Rust's performance, memory safety, and concurrency make it ideal for web applications, APIs, and backend services requiring high throughput.",
        ),
        (
            "Do you offer Rust training for our development team?",
            "Yes, we provide customized Rust training programs and workshops to quickly upskill teams in modern Rust practices.",
        ),
    ];
    let title = "Expert Rust Consulting Services | High-Performance Rust Development";
    let description = "Professional Rust consulting services specializing in high-performance, reliable, and scalable Rust development.";
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

        div { class: "min-h-screen text-white bg-[#1e1e1e]",
            section { class: "px-4 pt-12 pb-24 sm:px-6 lg:px-8",
                div { class: "mx-auto max-w-5xl",
                    h1 { class: "mb-6 text-5xl font-extrabold leading-tight sm:text-6xl md:text-7xl text-[#ffef5c]",
                        "Rust Consulting & Development"
                        br {}
                        "for the Modern Era"
                    }
                    p { class: "mb-8 max-w-2xl text-xl text-gray-300",
                        "Expert Rust consultants crafting high-performance, reliable, and scalable systems tailored to your business needs."
                    }
                    Link {
                        to: Route::References {},
                        class: "inline-flex items-center text-lg font-semibold hover:underline text-[#ffef5c]",
                        "Explore Our Rust Projects"
                        svg {
                            class: "ml-2 size-5",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M17 8l4 4m0 0l-4 4m4-4H3"
                            }
                        }
                    }
                }
            }

            section { class: "py-20 px-4 sm:px-6 lg:px-8 bg-[#2a2a2a]",
                div { class: "mx-auto max-w-5xl",
                    h2 { class: "mb-12 text-3xl font-bold text-[#ffef5c]", "Our Rust Expertise" }
                    div { class: "grid grid-cols-1 gap-12 md:grid-cols-2",
                        div {
                            h3 { class: "mb-4 text-2xl font-semibold text-white", "Rust Consulting & Development" }
                            p { class: "mb-6 text-gray-300", "Efficient, safe, and concurrent systems built with Rust. Specialized consulting for web services, embedded systems, and high-performance applications." }
                        }
                        div {
                            h3 { class: "mb-4 text-2xl font-semibold text-white", "Advanced System Architecture" }
                            p { class: "mb-6 text-gray-300", "Designing robust architectures to ensure your Rust applications are performant, scalable, and future-proof." }
                        }
                    }
                }
            }

            section { class: "py-20 px-4 sm:px-6 lg:px-8",
                div { class: "mx-auto max-w-5xl",
                    h2 { class: "mb-12 text-3xl font-bold text-[#ffef5c]", "Meet Our Rust Experts" }
                    div { class: "grid grid-cols-1 gap-12 md:grid-cols-2",
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
            }

            section { class: "py-20 px-4 sm:px-6 lg:px-8 bg-[#2a2a2a]",
                div { class: "mx-auto max-w-3xl",
                    h2 { class: "mb-8 text-3xl font-bold text-[#ffef5c]", "Get In Touch" }
                    form {
                        class: "space-y-6",
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
                                            "Message sent successfully! We'll get back to you shortly.".to_string(),
                                        ));
                                    }
                                    Err(err) => status.set(Some(format!("Failed to send message: {err}"))),
                                }
                            });
                        },
                        div { class: "grid grid-cols-1 gap-6 md:grid-cols-2",
                            input {
                                placeholder: "Your Name",
                                r#type: "text",
                                value: "{name()}",
                                oninput: move |event| name.set(event.value()),
                                class: "py-3 px-4 w-full placeholder-gray-400 text-white transition-shadow focus:ring-2 focus:outline-none bg-[#1e1e1e] focus:ring-[#ffef5c]"
                            }
                            input {
                                placeholder: "Your Email",
                                r#type: "email",
                                value: "{email()}",
                                oninput: move |event| email.set(event.value()),
                                class: "py-3 px-4 w-full placeholder-gray-400 text-white transition-shadow focus:ring-2 focus:outline-none bg-[#1e1e1e] focus:ring-[#ffef5c]"
                            }
                        }
                        input {
                            placeholder: "Subject",
                            r#type: "text",
                            value: "{subject()}",
                            oninput: move |event| subject.set(event.value()),
                            class: "py-3 px-4 w-full placeholder-gray-400 text-white transition-shadow focus:ring-2 focus:outline-none bg-[#1e1e1e] focus:ring-[#ffef5c]"
                        }
                        textarea {
                            placeholder: "Your Message",
                            rows: 6,
                            value: "{message()}",
                            oninput: move |event| message.set(event.value()),
                            class: "py-3 px-4 w-full placeholder-gray-400 text-white transition-shadow focus:ring-2 focus:outline-none bg-[#1e1e1e] focus:ring-[#ffef5c]"
                        }
                        button {
                            r#type: "submit",
                            disabled: *sending.read(),
                            class: "flex justify-center items-center py-3 px-6 w-full text-lg font-semibold transition-colors bg-[#ffef5c] text-[#1e1e1e] hover:bg-[#ffef5c]/90 disabled:opacity-70",
                            if *sending.read() {
                                svg {
                                    class: "w-6 h-6 animate-spin fill-black",
                                    view_box: "0 0 100 101",
                                    fill: "none",
                                    path {
                                        d: "M93.97 39.04c2.42-.63 3.89-3.13 3.04-5.48-1.71-4.73-4.13-9.18-7.19-13.2-3.97-5.23-8.93-9.62-14.6-12.94-5.67-3.31-11.94-5.47-18.44-6.36-5-.69-10.07-.61-15.03.23-2.47.41-3.92 2.92-3.28 5.35.64 2.42 3.12 3.85 5.6 3.49 3.8-.56 7.67-.58 11.49-.06 5.32.73 10.45 2.5 15.09 5.21 4.64 2.71 8.7 6.31 11.95 10.59 2.33 3.07 4.21 6.45 5.59 10.04.9 2.34 3.36 3.8 5.79 3.13Z",
                                        fill: "currentColor"
                                    }
                                }
                            } else {
                                "Send Message"
                            }
                        }
                        if let Some(current_status) = status() {
                            p { class: "text-[#ffef5c]", "{current_status}" }
                        }
                    }
                }
            }

            section { class: "py-24 px-4 sm:px-6 lg:px-8 bg-[#2a2a2a]",
                div { class: "mx-auto max-w-4xl",
                    div { class: "text-center mb-16",
                        h2 { class: "text-4xl font-bold text-[#ffef5c] mb-4", "Rust Consulting FAQ" }
                        p { class: "text-xl text-gray-300", "Common questions about our Rust consulting services" }
                    }
                    div { class: "space-y-4",
                        for (index, faq) in faqs.iter().enumerate() {
                            article { class: "bg-[#1e1e1e] border border-transparent overflow-hidden hover:border-[#ffef5c]/30 transition-colors",
                                button {
                                    class: "w-full px-8 py-6 text-left flex items-center justify-between hover:bg-[#2a2a2a]/50 transition-colors",
                                    onclick: move |_| {
                                        if open_faq() == Some(index) {
                                            open_faq.set(None);
                                        } else {
                                            open_faq.set(Some(index));
                                        }
                                    },
                                    h3 { class: "text-xl font-semibold text-white pr-4", "{faq.0}" }
                                    span { class: "text-xl text-white", if open_faq() == Some(index) { "⌄" } else { "⌃" } }
                                }
                                if open_faq() == Some(index) {
                                    div { class: "px-8 pb-6",
                                        p { class: "text-gray-300 leading-relaxed text-lg", "{faq.1}" }
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
fn ExpertCard(name: &'static str, role: &'static str, image: &'static str, linkedin: &'static str) -> Element {
    rsx! {
        div { class: "flex items-center space-x-6",
            img { src: "{image}", alt: "{name}", width: 100, height: 100, class: "rounded-full" }
            div {
                h3 { class: "mb-1 text-xl font-semibold text-white", "{name}" }
                p { class: "mb-2 text-gray-300", "{role}" }
                a {
                    href: "{linkedin}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "text-sm hover:underline text-[#ffef5c]",
                    "LinkedIn Profile"
                }
            }
        }
    }
}
