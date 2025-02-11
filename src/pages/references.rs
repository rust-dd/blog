use crate::ssr::api::select_references;
use leptos::{
    html::{div, h1, h3, p, section, span},
    prelude::*,
};

pub fn component() -> impl IntoView {
    let references = Resource::new_blocking(|| (), move |_| async move { select_references().await });

    div().class("container py-12 px-4 mx-auto").child((
        section().id("about").class("mx-auto mb-16 max-w-4xl text-left").child((
            h1().class("mb-8 text-5xl font-bold md:text-7xl text-[#ffef5c]").child("Our Project References"),
            p().class("mb-8 text-lg text-gray-300 md:text-xl").child("Explore our portfolio of successful projects. We specialize in building high-performance, reliable systems that make a real impact."),
        )),
        section().id("projects").class("mx-auto max-w-5xl").child(
            div().class("grid gap-8").child(
                Suspense(
                    SuspenseProps::builder()
                        .fallback(|| ())
                        .children(TypedChildren::to_children(move || {
                            For(
                                ForProps::builder()
                                    .each(move || references.get().and_then(Result::ok).unwrap_or_default())
                                    .key(|r| r.id.id.to_string())
                                    .children(|r| {
                                        div().class("relative p-6 rounded-2xl transition-colors duration-500 group bg-[#ffef5c]/8 hover:bg-[#ffef5c]/10").child((
                                            div().class("absolute inset-0 rounded-2xl -z-10 blur-2xl"),
                                            div().class("absolute inset-2 rounded-xl border shadow-lg -z-10 bg-[#ffef5c]/10 backdrop-blur-xl shadow-[#ffef5c]/5 border-[#ffef5c]/20"),
                                            div().class("absolute inset-2 rounded-xl border -z-10 backdrop-blur-2xl bg-white/5 border-white/10").child(
                                                div().class("absolute inset-0 bg-[linear-gradient(0deg,transparent_24px,rgba(255,255,255,0.03)_25px),linear-gradient(90deg,transparent_24px,rgba(255,255,255,0.03)_25px)] bg-[size:25px_25px]"),
                                            ),
                                            div().class("flex relative flex-col").child((
                                                h3().class("mb-2 text-xl font-bold text-[#ffef5c]").child(r.title),
                                                p().class("flex-grow mb-4 text-sm text-gray-300").child(r.description),
                                                div().class("grid grid-cols-2 gap-4").child(
                                                    For(
                                                        ForProps::builder()
                                                            .each(move || {
                                                                r.tech_stack
                                                                    .clone()
                                                                    .into_iter()
                                                                    .zip(r.teck_stack_percentage.clone().into_iter())
                                                                    .collect::<Vec<_>>()
                                                            })
                                                            .key(|tech| tech.0.to_string())
                                                            .children(|tech| {
                                                                div().child(
                                                                    (
                                                                        div().class("flex justify-between items-center mb-1")
                                                                            .child((
                                                                                span().class("text-xs font-medium text-[#ffef5c]").child(tech.0.to_string()),
                                                                                span().class("text-xs text-gray-400").child(format!("{}%", tech.1))
                                                                            )),
                                                                        div().class("overflow-hidden h-1.5 rounded-full bg-black/40 backdrop-blur-sm")
                                                                            .child(
                                                                                div()
                                                                                    .class("h-full bg-gradient-to-r from-[#ffef5c] to-[#ffef5c]")
                                                                                    .style(format!("width: {}%", tech.1.min(100))),
                                                                            )
                                                                ))
                                                            })
                                                            .build(),
                                                    ),
                                                ),
                                            )),
                                        ))
                                    })
                                    .build()
                            )
                        }))
                        .build()
                )
            )
        )
    ))
}
