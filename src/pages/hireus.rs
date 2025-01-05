use leptos::{
    ev,
    html::{a, br, button, div, form, h1, h2, h3, img, input, p, section, textarea},
    prelude::*,
    svg::{path, svg},
};

use crate::ssr::api::{hire_us, HireUsRequest};

pub fn component() -> impl IntoView {
    let state = RwSignal::new(HireUsRequest::default());
    let (sent, set_sent) = signal(false);
    let (loader, set_loader) = signal(false);
    let submit = Action::new(move |data: &HireUsRequest| {
        set_loader(true);
        let data = data.clone();

        async move {
            let _ = hire_us(data).await;
            state.set(HireUsRequest::default());
            set_sent(true);
            set_loader(false);
        }
    });

    div().class("min-h-screen text-white bg-[#1e1e1e]").child((
    section().class("px-4 pt-12 pb-24 sm:px-6 lg:px-8").child(
        div().class("mx-auto max-w-5xl").child((
            h1().class("mb-6 text-5xl font-extrabold leading-tight sm:text-6xl md:text-7xl text-[#ffef5c]").child(("Rust Development", br(), "for the Modern Era")),
            p().class("mb-8 max-w-2xl text-xl text-gray-300").child("We craft high-performance, reliable systems with Rust. Elevating your projects with cutting-edge technology and expert development."),
            a().href("/references").class("inline-flex items-center text-lg font-semibold hover:underline text-[#ffef5c]").child((
            "Explore Our Work",
            svg().class("ml-2 size-5").attr("fill", "none").attr("stroke", "currentColor").attr("viewBox", "0 0 24 24").attr("xmlns", "http://www.w3.org/2000/svg").child(
                path().attr("stroke-linecap", "round").attr("stroke-linejoin", "round").attr("stroke-width", 2).attr("d", "M17 8l4 4m0 0l-4 4m4-4H3"),
            ),
        )),
    ))),
    section().class("py-20 px-4 sm:px-6 lg:px-8 bg-[#2a2a2a]").child(
        div().class("mx-auto max-w-5xl").child((
            h2().class("mb-12 text-3xl font-bold text-[#ffef5c]").child("Our Expertise"),
            div().class("grid grid-cols-1 gap-12 md:grid-cols-2").child((
                div().child((
                    h3().class("mb-4 text-2xl font-semibold text-white").child("Rust Development"),
                    p().class("mb-6 text-gray-300").child("Specializing in efficient, safe, and concurrent systems. From low-level programming to high-performance web services, we leverage Rust's power to deliver exceptional solutions."),
                )),
                div().child((
                    h3().class("mb-4 text-2xl font-semibold text-white").child("System Architecture"),
                    p().class("mb-6 text-gray-300").child("Designing robust and scalable architectures that stand the test of time. Our expertise ensures your systems are built for performance, reliability, and future growth."),
                )),
            )),
        ))
    ),
    section().class("py-20 px-4 sm:px-6 lg:px-8").child(
        div().class("mx-auto max-w-5xl").child((
            h2().class("mb-12 text-3xl font-bold text-[#ffef5c]").child("Meet Our Team"),
            div().class("grid grid-cols-1 gap-12 md:grid-cols-2").child((
                div().class("flex items space-x-6").child((
                    img().src("https://static.rust-dd.com/zelei.webp").alt("Daniel Zelei").width(100).height(100).class("rounded-full"),
                    div().child((
                        h3().class("mb-1 text-xl font-semibold text-white").child("Daniel Zelei"),
                        p().class("mb-2 text-gray-300").child("Senior Software Engineer"),
                        a().href("https://www.linkedin.com/in/danielzelei/").target("_blank").rel("noopener noreferrer").class("text-sm hover:underline text-[#ffef5c]").child("LinkedIn Profile"),
                    )),
                )),
                div().class("flex items-center space-x-6").child((
                    img().src("https://static.rust-dd.com/boros.webp").alt("Daniel Boros").width(100).height(100).class("rounded-full"),
                    div().child((
                        h3().class("mb-1 text-xl font-semibold text-white").child("Daniel Boros"),
                        p().class("mb-2 text-gray-300").child("Senior Software Engineer"),
                        a().href("https://www.linkedin.com/in/daniel-boros-b86a5373/").target("_blank").rel("noopener noreferrer").class("text-sm hover:underline text-[#ffef5c]").child("LinkedIn Profile"),
                    )),
                )),
        ))
    ))),
     section().class("py-20 px-4 sm:px-6 lg:px-8 bg-[#2a2a2a]").child(
         div().class("mx-auto max-w-3xl").child((
             h2().class("mb-8 text-3xl font-bold text-[#ffef5c]").child("Get In Touch"),
             form().class("space-y-6").on(ev::submit ,move |e| {
                 e.prevent_default();
                 let _ = submit.dispatch(state.get());
             }).child((
                 div().class("grid grid-cols-1 gap-6 md:grid-cols-2").child((
                     input().placeholder("Your Name").attr("type", "text").prop("value", move || state.get().name).on(ev::input, move |e| {
                         let name = event_target_value(&e);
                         state.update(|prev| { prev.name = name });
                     }).class("py-3 px-4 w-full placeholder-gray-400 text-white transition-shadow focus:ring-2 focus:outline-none bg-[#1e1e1e] focus:ring-[#ffef5c]"),
                     input().placeholder("Your Email").attr("type", "text").prop("value", move || state.get().email).on(ev::input, move |e| {
                         let email = event_target_value(&e);
                         state.update(|prev| { prev.email = email });
                     }).class("py-3 px-4 w-full placeholder-gray-400 text-white transition-shadow focus:ring-2 focus:outline-none bg-[#1e1e1e] focus:ring-[#ffef5c]"),
                 )),
                 input().placeholder("Subject").attr("type", "text").prop("value", move || state.get().subject).on(ev::input, move |e| {
                     let subject = event_target_value(&e);
                     state.update(|prev| { prev.subject = subject });
                 }).class("py-3 px-4 w-full placeholder-gray-400 text-white transition-shadow focus:ring-2 focus:outline-none bg-[#1e1e1e] focus:ring-[#ffef5c]"),
                 textarea().placeholder("Your Message").prop("value", move || state.get().message).on(ev::input, move |e| {
                     let message = event_target_value(&e);
                     state.update(|prev| { prev.message = message });
                 }).rows(6).class("py-3 px-4 w-full placeholder-gray-400 text-white transition-shadow focus:ring-2 focus:outline-none bg-[#1e1e1e] focus:ring-[#ffef5c]"),
                 button().attr("type", "submit").class("flex justify-center items-center py-3 px-6 w-full text-lg font-semibold transition-colors bg-[#ffef5c] text-[#1e1e1e] hover:bg-[#ffef5c]/90").child(
                     Show(ShowProps::builder().when(loader).fallback(|| "Send Message").children(ToChildren::to_children(move || {
                         svg().class("w-8 h-8 animate-spin fill-black").attr("aria-hidden", "true").attr("viewBox", "0 0 100 101").attr("fill", "none").attr("xmlns", "http://www.w3.org/2000/svg").child(
                             path().attr("d", "M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z").attr("fill", "currentFill")
                         )
                     })).build())),
                Show(ShowProps::builder().when(sent).fallback(|| ()).children(ToChildren::to_children(|| p().class("text-[#ffef5c]").child("Message sent successfully! We\'ll get back to you shortly."))).build()
             ),
            )),
         ))
     ),
  ))
}
