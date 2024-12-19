use leptos::prelude::*;

use crate::ssr::api::{hire_us, HireUsRequest};

#[component]
pub fn Component() -> impl IntoView {
    let state = RwSignal::new(HireUsRequest::default());
    let (sent, set_sent) = signal(false);
    let submit = Action::new(move |data: &HireUsRequest| {
        let data = data.clone();
        async move {
            let _ = hire_us(data).await;
            state.set(HireUsRequest::default());
            set_sent(true);
        }
    });

    view! {
        <div class="min-h-screen text-white bg-[#1e1e1e]">
            <section class="px-4 pt-12 pb-24 sm:px-6 lg:px-8">
                <div class="mx-auto max-w-5xl">
                    <h1 class="mb-6 text-5xl font-extrabold leading-tight sm:text-6xl md:text-7xl text-[#ffef5c]">
                        Rust Development <br />for the Modern Era
                    </h1>
                    <p class="mb-8 max-w-2xl text-xl text-gray-300">
                        We craft high-performance, reliable systems with Rust. Elevating your projects with cutting-edge technology and expert development.
                    </p>
                    <a
                        href="/references"
                        class="inline-flex items-center text-lg font-semibold hover:underline text-[#ffef5c]"
                    >
                        Explore Our Work
                        <svg
                            class="ml-2 w-5 h-5"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                strokeLinecap="round"
                                strokeLinejoin="round"
                                strokeWidth=2
                                d="M17 8l4 4m0 0l-4 4m4-4H3"
                            />
                        </svg>
                    </a>
                </div>
            </section>
            <section class="py-20 px-4 sm:px-6 lg:px-8 bg-[#2a2a2a]">
                <div class="mx-auto max-w-5xl">
                    <h2 class="mb-12 text-3xl font-bold text-[#ffef5c]">Our Expertise</h2>
                    <div class="grid grid-cols-1 gap-12 md:grid-cols-2">
                        <div>
                            <h3 class="mb-4 text-2xl font-semibold text-white">Rust Development</h3>
                            <p class="mb-6 text-gray-300">
                                {"Specializing in efficient, safe, and concurrent systems. From low-level programming to high-performance web services, we leverage Rust's power to deliver exceptional solutions."}
                            </p>
                        </div>
                        <div>
                            <h3 class="mb-4 text-2xl font-semibold text-white">
                                System Architecture
                            </h3>
                            <p class="mb-6 text-gray-300">
                                Designing robust and scalable architectures that stand the test of time. Our expertise ensures your systems are built for performance, reliability, and future growth.
                            </p>
                        </div>
                    </div>
                </div>
            </section>
            <section class="py-20 px-4 sm:px-6 lg:px-8">
                <div class="mx-auto max-w-5xl">
                    <h2 class="mb-12 text-3xl font-bold text-[#ffef5c]">Meet Our Team</h2>
                    <div class="grid grid-cols-1 gap-12 md:grid-cols-2">
                        <div class="flex items-center space-x-6">
                            <img
                                src="https://static.rust-dd.com/zelei.webp"
                                alt="Daniel Zelei"
                                width=100
                                height=100
                                class="rounded-full"
                            />
                            <div>
                                <h3 class="mb-1 text-xl font-semibold text-white">Daniel Zelei</h3>
                                <p class="mb-2 text-gray-300">Senior Software Engineer</p>
                                <a
                                    href="https://www.linkedin.com/in/danielzelei/"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="text-sm hover:underline text-[#ffef5c]"
                                >
                                    LinkedIn Profile
                                </a>
                            </div>
                        </div>
                        <div class="flex items-center space-x-6">
                            <img
                                src="https://static.rust-dd.com/boros.webp"
                                alt="Daniel Boros"
                                width=100
                                height=100
                                class="rounded-full"
                            />
                            <div>
                                <h3 class="mb-1 text-xl font-semibold text-white">Daniel Boros</h3>
                                <p class="mb-2 text-gray-300">Senior Software Engineer</p>
                                <a
                                    href="https://www.linkedin.com/in/daniel-boros-b86a5373/"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="text-sm hover:underline text-[#ffef5c]"
                                >
                                    LinkedIn Profile
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            <section class="py-20 px-4 sm:px-6 lg:px-8 bg-[#2a2a2a]">
                <div class="mx-auto max-w-3xl">
                    <h2 class="mb-8 text-3xl font-bold text-[#ffef5c]">Get In Touch</h2>
                    <form
                        class="space-y-6"
                        on:submit=move |e| {
                            e.prevent_default();
                            let _ = submit.dispatch(state.get());
                        }
                    >
                        <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
                            <input
                                type="text"
                                placeholder="Your Name"
                                on:input=move |e| {
                                    let name = event_target_value(&e);
                                    state.update(|prev| { prev.name = name });
                                }
                                class="py-3 px-4 w-full placeholder-gray-400 text-white transition-shadow focus:ring-2 focus:outline-none bg-[#1e1e1e] focus:ring-[#ffef5c]"
                            />
                            <input
                                type="email"
                                placeholder="Your Email"
                                on:input=move |e| {
                                    let email = event_target_value(&e);
                                    state.update(|prev| { prev.email = email });
                                }
                                class="py-3 px-4 w-full placeholder-gray-400 text-white transition-shadow focus:ring-2 focus:outline-none bg-[#1e1e1e] focus:ring-[#ffef5c]"
                            />
                        </div>
                        <input
                            type="text"
                            placeholder="Subject"
                            on:input=move |e| {
                                let subject = event_target_value(&e);
                                state.update(|prev| { prev.subject = subject });
                            }
                            class="py-3 px-4 w-full placeholder-gray-400 text-white transition-shadow focus:ring-2 focus:outline-none bg-[#1e1e1e] focus:ring-[#ffef5c]"
                        />
                        <textarea
                            placeholder="Your Message"
                            on:input=move |e| {
                                let message = event_target_value(&e);
                                state.update(|prev| { prev.message = message });
                            }
                            rows=6
                            class="py-3 px-4 w-full placeholder-gray-400 text-white transition-shadow focus:ring-2 focus:outline-none bg-[#1e1e1e] focus:ring-[#ffef5c]"
                        ></textarea>
                        <button
                            type="submit"
                            class="py-3 px-6 w-full text-lg font-semibold transition-colors bg-[#ffef5c] text-[#1e1e1e] hover:bg-[#ffef5c]/90"
                        >
                            Send Message
                        </button>
                        <Show when=sent fallback=|| ()>
                            <p class="text-[#ffef5c] text-center">
                                {"Message sent successfully! We\'ll get back to you shortly."}
                            </p>
                        </Show>
                    </form>
                </div>
            </section>
        </div>
    }
}
