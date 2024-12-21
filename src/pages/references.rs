use crate::ssr::api::select_references;
use leptos::prelude::*;

#[component]
pub fn Component() -> impl IntoView {
    let references =
        Resource::new_blocking(|| (), move |_| async move { select_references().await });

    view! {
        <div class="container py-12 px-4 mx-auto">
            <section class="mx-auto mb-16 max-w-4xl text-center">
                <h1 class="mb-8 text-5xl font-bold md:text-7xl text-[#ffef5c]">
                    Our Project References
                </h1>
                <p class="mb-8 text-lg text-gray-300 md:text-xl">
                    Explore our portfolio of successful projects. We specialize in building high-performance,
                    reliable systems that make a real impact.
                </p>
            </section>
            <section id="projects" class="mx-auto max-w-5xl">
                <div class="grid gap-8">
                    <Suspense fallback=|| ()>
                        <For
                            each=move || references.get().and_then(Result::ok).unwrap_or_default()
                            key=|r| r.id.id.to_string()
                            let:reference
                        >
                            <div class="relative h-[300px] group">
                                <div class="absolute inset-0 z-0 rounded-2xl transition-colors duration-500 bg-[#ffef5c]/8 blur-2xl group-hover:bg-[#ffef5c]/10"></div>
                                <div class="absolute inset-2 z-10 rounded-xl border shadow-lg bg-[#ffef5c]/10 backdrop-blur-xl shadow-[#ffef5c]/5 border-[#ffef5c]/20"></div>
                                <div class="overflow-hidden absolute inset-2 z-20 rounded-xl border backdrop-blur-2xl bg-white/5 border-white/10">
                                    <div class="absolute inset-0 bg-[linear-gradient(0deg,transparent_24px,rgba(255,255,255,0.03)_25px),linear-gradient(90deg,transparent_24px,rgba(255,255,255,0.03)_25px)] bg-[size:25px_25px]"></div>
                                </div>
                                <div class="flex absolute inset-0 z-30 flex-col px-6 pt-6 pb-10">
                                    <h3 class="mb-2 text-xl font-bold text-[#ffef5c]">
                                        {reference.title}
                                    </h3>
                                    <p class="flex-grow mb-4 text-sm text-gray-300">
                                        {reference.description}
                                    </p>
                                    <div class="grid grid-cols-2 gap-4">
                                        <For
                                            each=move || {
                                                reference
                                                    .tech_stack
                                                    .clone()
                                                    .into_iter()
                                                    .zip(reference.teck_stack_percentage.clone().into_iter())
                                                    .collect::<Vec<_>>()
                                            }
                                            key=|tech| tech.0.to_string()
                                            let:tech
                                        >
                                            <div class="flex justify-between items-center mb-1">
                                                <span class="text-xs font-medium text-[#ffef5c]">
                                                    {tech.0.to_string()}
                                                </span>
                                                <span class="text-xs text-gray-400">{tech.1}%</span>
                                            </div>
                                            <div class="overflow-hidden h-1.5 rounded-full bg-black/40 backdrop-blur-sm">
                                                <div
                                                    class="h-full bg-gradient-to-r from-[#ffef5c] to-[#ffef5c]"
                                                    style=format!("width: {}%", tech.1.min(100))
                                                ></div>
                                            </div>
                                        </For>
                                    </div>
                                </div>
                            </div>
                        </For>
                    </Suspense>
                </div>
            </section>
        </div>
    }
}
