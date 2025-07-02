use icondata as i;
use leptos::{
    ev,
    html::{a, br, button, div, form, h1, h2, h3, img, input, meta, p, section, textarea, title},
    prelude::*,
    svg::{path, svg},
};
use leptos_icons::{Icon, IconProps};

use crate::ssr::api::{hire_us, HireUsRequest};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Faq {
    id: u32,
    question: String,
    answer: String,
    is_open: RwSignal<bool>,
}

pub fn component() -> impl IntoView {
    let state = RwSignal::new(HireUsRequest::default());
    let (sent, set_sent) = signal(false);
    let (loader, set_loader) = signal(false);
    let faqs = RwSignal::new(vec![
        Faq {
            id: 1,
            question: "Why choose Rust for your next project?".to_string(),
            answer: "Rust ensures performance, safety, and reliability, ideal for system-critical applications, embedded solutions, and high-performance computing.".to_string(),
            is_open: RwSignal::new(false),
        },
        Faq {
            id: 2,
            question: "What Rust consulting services do we offer?".to_string(),
            answer: "We offer Rust consulting, architecture design, performance optimization, code audits, training, and custom Rust development solutions.".to_string(),
            is_open: RwSignal::new(false),
        },
        Faq {
            id: 3,
            question: "How can Rust consulting benefit my business?".to_string(),
            answer: "Our expert Rust consultants help businesses build faster, safer, and more scalable software solutions, reducing technical debt and ensuring long-term reliability.".to_string(),
            is_open: RwSignal::new(false),
        },
        Faq {
            id: 4,
            question: "Is Rust suitable for web development?".to_string(),
            answer: "Rust's performance, memory safety, and concurrency make it ideal for web applications, APIs, and backend services requiring high throughput and reliability.".to_string(),
            is_open: RwSignal::new(false),
        },
        Faq {
            id: 5,
            question: "Do you offer Rust training for our development team?".to_string(),
            answer: "Yes, we provide customized Rust training programs and workshops to quickly upskill your team in modern Rust development practices.".to_string(),
            is_open: RwSignal::new(false),
        },
    ]);

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
        title().child("Expert Rust Consulting Services | High-Performance Rust Development"),
        meta().attr("name", "description").attr("content", "Professional Rust consulting services specializing in high-performance, reliable, and scalable Rust development."),
        section().class("px-4 pt-12 pb-24 sm:px-6 lg:px-8").child(
            div().class("mx-auto max-w-5xl").child((
                h1().class("mb-6 text-5xl font-extrabold leading-tight sm:text-6xl md:text-7xl text-[#ffef5c]")
                    .child(("Rust Consulting & Development", br(), "for the Modern Era")),
                p().class("mb-8 max-w-2xl text-xl text-gray-300").child("Expert Rust consultants crafting high-performance, reliable, and scalable systems tailored to your business needs."),
                a().href("/references").class("inline-flex items-center text-lg font-semibold hover:underline text-[#ffef5c]").child((
                    "Explore Our Rust Projects",
                    svg().class("ml-2 size-5").attr("fill", "none").attr("stroke", "currentColor").attr("viewBox", "0 0 24 24").attr("xmlns", "http://www.w3.org/2000/svg").child(
                        path().attr("stroke-linecap", "round").attr("stroke-linejoin", "round").attr("stroke-width", 2).attr("d", "M17 8l4 4m0 0l-4 4m4-4H3"),
                    ),
                )),
            )),
        ),
        section().class("py-20 px-4 sm:px-6 lg:px-8 bg-[#2a2a2a]").child(
            div().class("mx-auto max-w-5xl").child((
                h2().class("mb-12 text-3xl font-bold text-[#ffef5c]").child("Our Rust Expertise"),
                div().class("grid grid-cols-1 gap-12 md:grid-cols-2").child((
                    div().child((
                        h3().class("mb-4 text-2xl font-semibold text-white").child("Rust Consulting & Development"),
                        p().class("mb-6 text-gray-300").child("Efficient, safe, and concurrent systems built with Rust. Specialized consulting for web services, embedded systems, and high-performance applications."),
                    )),
                    div().child((
                        h3().class("mb-4 text-2xl font-semibold text-white").child("Advanced System Architecture"),
                        p().class("mb-6 text-gray-300").child("Designing robust architectures to ensure your Rust applications are performant, scalable, and future-proof."),
                    )),
                )),
            )),
        ),
        section().class("py-20 px-4 sm:px-6 lg:px-8").child(
            div().class("mx-auto max-w-5xl").child((
                h2().class("mb-12 text-3xl font-bold text-[#ffef5c]").child("Meet Our Rust Experts"),
                div().class("grid grid-cols-1 gap-12 md:grid-cols-2").child((
                    div().class("flex items-center space-x-6").child((
                        img().src("https://static.rust-dd.com/zelei.webp").alt("Daniel Zelei").width(100).height(100).class("rounded-full"),
                        div().child((
                            h3().class("mb-1 text-xl font-semibold text-white").child("Daniel Zelei"),
                            p().class("mb-2 text-gray-300").child("Senior Rust Consultant"),
                            a().href("https://www.linkedin.com/in/danielzelei/").target("_blank").rel("noopener noreferrer").class("text-sm hover:underline text-[#ffef5c]").child("LinkedIn Profile"),
                        )),
                    )),
                    div().class("flex items-center space-x-6").child((
                        img().src("https://static.rust-dd.com/boros.webp").alt("Daniel Boros").width(100).height(100).class("rounded-full"),
                        div().child((
                            h3().class("mb-1 text-xl font-semibold text-white").child("Daniel Boros"),
                            p().class("mb-2 text-gray-300").child("Senior Rust Consultant"),
                            a().href("https://www.linkedin.com/in/daniel-boros-b86a5373/").target("_blank").rel("noopener noreferrer").class("text-sm hover:underline text-[#ffef5c]").child("LinkedIn Profile"),
                        )),
                    )),
                ))
            ))
        ),

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

     section()
         .class("py-24 px-4 sm:px-6 mt-28 absolute left-0 right-0 lg:px-8 bg-[#2a2a2a]")
         .child(
             div()
                 .class("mx-auto max-w-4xl")
                 .child((
                     div()
                         .class("text-center mb-16")
                         .child((
                             h2()
                                 .class("text-4xl font-bold text-[#ffef5c] mb-4")
                                 .child("Rust Consulting FAQ"),
                             p()
                                 .class("text-xl text-gray-300")
                                 .child("Common questions about our Rust consulting services"),
                         )),
                     div()
                         .class("space-y-4")
                         .child(
                             For(
                                 ForProps::builder()
                                     .each(move || faqs.read().clone().into_iter())
                                     .key(|item| item.id.to_string())
                                     .children(move |item| {
                                         div()
                                             .class("bg-[#1e1e1e] border border-transparent overflow-hidden hover:border-[#ffef5c]/30 transition-colors")
                                             .child((
                                                 button()
                                                     .on(ev::click,
                                                         move |_| {
                                                             item.is_open.update(|p| *p = !*p);
                                                     })
                                                     .class("w-full px-8 py-6 text-left flex items-center justify-between hover:bg-[#2a2a2a]/50 transition-colors")
                                                     .child((
                                                         h3()
                                                             .class("text-xl font-semibold text-white pr-4")
                                                             .child(item.question.clone()),
                                                         Show(ShowProps::builder()
                                                             .when(move || item.is_open.get())
                                                             .fallback(|| div().child(Icon(IconProps::builder()
                                                                 .icon(Signal::from(i::FaChevronUpSolid))
                                                                 .width("1em")
                                                                 .height("1em")
                                                                 .style("color: white")
                                                                 .build(),
                                                            )))
                                                             .children(ToChildren::to_children(move || {
                                                                 div().child(Icon(IconProps::builder()
                                                                     .icon(Signal::from(i::FaChevronDownSolid))
                                                                     .width("1em")
                                                                     .height("1em")
                                                                     .style("color: white")
                                                                     .build(),
                                                                ))
                                                             })).build()
                                                         )
                                                     )),
                                                 Show(ShowProps::builder()
                                                     .when(move || item.is_open.get())
                                                     .fallback(|| ())
                                                     .children(ToChildren::to_children(move || {
                                                         div().class("px-8 pb-6").child(
                                                             p()
                                                                 .class("text-gray-300 leading-relaxed text-lg")
                                                                 .child(item.answer.clone())
                                                         )
                                                     }))
                                                     .build()
                                                 ),
                                             ))
                                     })
                                     .build(),
                             )
                         ),
                 )),
         )
  ))
}
