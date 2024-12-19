use leptos::prelude::*;

#[component]
pub fn Component() -> impl IntoView {
    view! {
        <header class="fixed top-0 right-0 left-0 z-10 py-6 px-4 md:px-6 bg-[#1e1e1e]/80 backdrop-blur-md">
            <div class="container mx-auto max-w-5xl">
                <div class="flex flex-row justify-between items-center text-white">
                    <div class="flex flex-row gap-4">
                        <a
                            href="/"
                            on:click=move |_| {
                                use web_sys::window;
                                let document = window().unwrap().document().unwrap();
                                if let Some(element) = document.get_element_by_id("giscus") {
                                    if let Some(parent) = element.parent_node() {
                                        parent.remove_child(&element).unwrap();
                                    }
                                }
                            }
                            class="text-lg font-bold transition-all duration-500 sm:text-3xl hover:text-[#ffef5c]"
                        >
                            blog
                        </a>
                        <a
                            href="/references"
                            class="text-lg font-bold transition-all duration-500 sm:text-3xl hover:text-[#ffef5c]"
                        >
                            references
                        </a>
                        <a
                            href="/hireus"
                            class="text-lg font-bold transition-all duration-500 sm:text-3xl hover:text-[#ffef5c]"
                        >
                            hire us
                        </a>
                    </div>
                    <div class="flex flex-row gap-3 items-center h-10">
                        <a
                            href="https://github.com/rust-dd/"
                            rel="noopener noreferrer"
                            target="_blank"
                            aria-label="GitHub"
                        >
                            <svg
                                class="transition-all duration-500 size-6 hover:text-[#ffef5c]"
                                width="1em"
                                height="1em"
                                viewBox="0 0 512 512"
                                fill="currentColor"
                                role="graphics-symbol"
                                data-hk="0-0-0-72"
                            >
                                <path d="M256,32C132.3,32,32,134.9,32,261.7c0,101.5,64.2,187.5,153.2,217.9a17.56,17.56,0,0,0,3.8.4c8.3,0,11.5-6.1,11.5-11.4,0-5.5-.2-19.9-.3-39.1a102.4,102.4,0,0,1-22.6,2.7c-43.1,0-52.9-33.5-52.9-33.5-10.2-26.5-24.9-33.6-24.9-33.6-19.5-13.7-.1-14.1,1.4-14.1h.1c22.5,2,34.3,23.8,34.3,23.8,11.2,19.6,26.2,25.1,39.6,25.1a63,63,0,0,0,25.6-6c2-14.8,7.8-24.9,14.2-30.7-49.7-5.8-102-25.5-102-113.5,0-25.1,8.7-45.6,23-61.6-2.3-5.8-10-29.2,2.2-60.8a18.64,18.64,0,0,1,5-.5c8.1,0,26.4,3.1,56.6,24.1a208.21,208.21,0,0,1,112.2,0c30.2-21,48.5-24.1,56.6-24.1a18.64,18.64,0,0,1,5,.5c12.2,31.6,4.5,55,2.2,60.8,14.3,16.1,23,36.6,23,61.6,0,88.2-52.4,107.6-102.3,113.3,8,7.1,15.2,21.1,15.2,42.5,0,30.7-.3,55.5-.3,63,0,5.4,3.1,11.5,11.4,11.5a19.35,19.35,0,0,0,4-.4C415.9,449.2,480,363.1,480,261.7,480,134.9,379.7,32,256,32Z"></path>
                            </svg>
                        </a>
                        <a
                            href="https://x.com/rust_dd"
                            rel="noopener noreferrer"
                            target="_blank"
                            aria-label="X"
                        >
                            <svg
                                class="transition-all duration-500 size-6 hover:text-[#ffef5c]"
                                width="1em"
                                height="1em"
                                viewBox="0 0 512 512"
                                fill="currentColor"
                                role="graphics-symbol"
                                data-hk="0-0-0-76"
                            >
                                <path d="M389.2 48h70.6L305.6 224.2 487 464H345L233.7 318.6 106.5 464H35.8L200.7 275.5 26.8 48H172.4L272.9 180.9 389.2 48zM364.4 421.8h39.1L151.1 88h-42L364.4 421.8z"></path>
                            </svg>
                        </a>
                        <a
                            href="https://www.linkedin.com/company/rust-dd"
                            rel="noopener noreferrer"
                            target="_blank"
                            aria-label="LinkedIn"
                        >
                            <svg
                                class="transition-all duration-500 size-6 hover:text-[#ffef5c]"
                                width="1em"
                                height="1em"
                                viewBox="0 0 448 512"
                                fill="currentColor"
                                role="graphics-symbol"
                                data-hk="0-0-0-80"
                            >
                                <path d="M416 32H31.9C14.3 32 0 46.5 0 64.3v383.4C0 465.5 14.3 480 31.9 480H416c17.6 0 32-14.5 32-32.3V64.3c0-17.8-14.4-32.3-32-32.3zM135.4 416H69V202.2h66.5V416zm-33.2-243c-21.3 0-38.5-17.3-38.5-38.5S80.9 96 102.2 96c21.2 0 38.5 17.3 38.5 38.5 0 21.3-17.2 38.5-38.5 38.5zm282.1 243h-66.4V312c0-24.8-.5-56.7-34.5-56.7-34.6 0-39.9 27-39.9 54.9V416h-66.4V202.2h63.7v29.2h.9c8.9-16.8 30.6-34.5 62.9-34.5 67.2 0 79.7 44.3 79.7 101.9V416z"></path>
                            </svg>
                        </a>
                        <a
                            href="/rss.xml"
                            rel="noopener noreferrer"
                            target="_blank"
                            aria-label="RSS"
                        >
                            <svg
                                class="transition-all duration-500 size-6 hover:text-[#ffef5c]"
                                width="1em"
                                height="1em"
                                viewBox="0 0 512 512"
                                fill="currentColor"
                                role="graphics-symbol"
                                data-hk="0-0-0-84"
                            >
                                <path d="M108.56,342.78a60.34,60.34,0,1,0,60.56,60.44A60.63,60.63,0,0,0,108.56,342.78Z"></path>
                                <path d="M48,186.67v86.55c52,0,101.94,15.39,138.67,52.11s52,86.56,52,138.67h86.66C325.33,312.44,199.67,186.67,48,186.67Z"></path>
                                <path d="M48,48v86.56c185.25,0,329.22,144.08,329.22,329.44H464C464,234.66,277.67,48,48,48Z"></path>
                            </svg>
                        </a>
                    </div>
                </div>
            </div>
        </header>
    }
}
