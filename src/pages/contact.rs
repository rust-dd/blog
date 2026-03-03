use dioxus::prelude::*;

use crate::{
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

    let title = "Contact Us | Rust-DD";
    let description =
        "Get in touch with the Rust-DD team for consulting, collaboration, or questions about our open source projects.";
    let canonical = seo::absolute_url("/contact");

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

        div { class: "w-full font-mono",
            section { class: "py-4",
                p { class: "text-xs text-slate-400", "// contact" }
                h1 { class: "mt-2 text-3xl font-semibold leading-tight text-slate-900 sm:text-4xl md:text-5xl",
                    "Contact Us"
                }
                p { class: "mt-3 max-w-2xl text-sm leading-relaxed text-slate-600",
                    "Have a question, want to collaborate, or need Rust consulting? Drop us a message."
                }
            }

            section { class: "mt-6 rounded-lg border border-slate-200 bg-white p-4 sm:p-6",
                h2 { class: "text-2xl font-semibold text-slate-900 sm:text-3xl", "Send a Message" }

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
        }
    }
}
