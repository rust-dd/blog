use leptos::prelude::*;

struct Technology {
  name: &'static str,
  percentage: u32,
}

struct Project {
  title: &'static str,
  description: &'static str,
  technologies: Vec<Technology>,
}

fn get_projects() -> Vec<Project> {
  vec![
    Project {
      title: "Rust-DD Open Source Community",
      description: "I founded and manage the Rust-DD open-source community, fostering development in the Rust ecosystem.",
      technologies: vec![
        Technology { name: "Community Building", percentage: 100 },
        Technology { name: "Open Source", percentage: 100 },
      ],
    },
    Project {
      title: "Stochastic-RS Library",
      description: "Stochastic-RS is a Rust library I created for generating fractional stochastic processes to provide synthetic data for machine learning models.",
      technologies: vec![
        Technology { name: "Rust", percentage: 80 },
        Technology { name: "Machine Learning", percentage: 20 },
      ],
    },
    Project {
      title: "Real-Time Stream Server for Drone Streams",
      description: "Developed a real-time stream server in Rust for managing drone video streams.",
      technologies: vec![
        Technology { name: "Tokio", percentage: 40 },
        Technology { name: "Axum", percentage: 30 },
        Technology { name: "GraphQL", percentage: 20 },
        Technology { name: "PostgreSQL", percentage: 10 },
      ],
    },
    Project {
      title: "Object Detection with Local LLM and YOLO/Detectron2",
      description: "Working on object detection leveraging local LLM combined with YOLO and Detectron2 for real-time drone image processing.",
      technologies: vec![
        Technology { name: "Tokio", percentage: 20 },
        Technology { name: "Axum", percentage: 15 },
        Technology { name: "YOLO", percentage: 25 },
        Technology { name: "GraphQL", percentage: 10 },
        Technology { name: "PostgreSQL", percentage: 10 },
        Technology { name: "Candle-RS", percentage: 10 },
        Technology { name: "OpenCV-Rust", percentage: 10 },
      ],
    },
    Project {
      title: "Big Data Processing in Rust",
      description: "Managing a big data processing system that handles over 50TB of data daily.",
      technologies: vec![
        Technology { name: "Tokio", percentage: 40 },
        Technology { name: "Polars", percentage: 40 },
        Technology { name: "Rayon", percentage: 20 },
      ],
    },
    Project {
      title: "IoT MQTT Server",
      description: "Building an IoT MQTT server for high-performance data streaming.",
      technologies: vec![
        Technology { name: "Tokio", percentage: 30 },
        Technology { name: "Axum", percentage: 25 },
        Technology { name: "GraphQL", percentage: 15 },
        Technology { name: "Rumqtt", percentage: 15 },
        Technology { name: "EMQX", percentage: 15 },
      ],
    },
  ]
}

#[component]
pub fn Component() -> impl IntoView {
  let projects = get_projects();

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
                <h2 class="mb-8 text-3xl font-bold text-[#ffef5c]">Featured Projects</h2>
                <div class="grid gap-8">
                    {projects
                        .into_iter()
                        .map(|project| {
                            view! {
                                <div class="relative h-[300px] group">
                                    {/* Háttér rétegek */}
                                    <div class="absolute inset-0 z-0 rounded-2xl transition-colors duration-500 bg-[#ffef5c]/8 blur-2xl group-hover:bg-[#ffef5c]/10"></div>
                                    <div class="absolute inset-2 z-10 rounded-xl border shadow-lg bg-[#ffef5c]/10 backdrop-blur-xl shadow-[#ffef5c]/5 border-[#ffef5c]/20"></div>
                                    <div class="overflow-hidden absolute inset-2 z-20 rounded-xl border backdrop-blur-2xl bg-white/5 border-white/10">
                                        <div class="absolute inset-0 bg-[linear-gradient(0deg,transparent_24px,rgba(255,255,255,0.03)_25px),linear-gradient(90deg,transparent_24px,rgba(255,255,255,0.03)_25px)] bg-[size:25px_25px]"></div>
                                    </div>

                                    {/* Tartalom */}
                                    <div class="flex absolute inset-0 z-30 flex-col px-6 pt-6 pb-10">
                                        <h3 class="mb-2 text-xl font-bold text-[#ffef5c]">{project.title}</h3>
                                        <p class="flex-grow mb-4 text-sm text-gray-300">
                                            {project.description}
                                        </p>
                                        <div class="grid grid-cols-2 gap-4">
                                            {project
                                                .technologies
                                                .into_iter()
                                                .map(|tech| {
                                                    view! {
                                                        <div>
                                                            <div class="flex justify-between items-center mb-1">
                                                                <span class="text-xs font-medium text-[#ffef5c]">
                                                                    {tech.name}
                                                                </span>
                                                                <span class="text-xs text-gray-400">
                                                                    {tech.percentage}%
                                                                </span>
                                                            </div>
                                                            <div class="overflow-hidden h-1.5 rounded-full bg-black/40 backdrop-blur-sm">
                                                                <div
                                                                    class="h-full bg-gradient-to-r from-[#ffef5c] to-[#ffef5c]"
                                                                    style=format!("width: {}%", tech.percentage.min(100))
                                                                ></div>
                                                            </div>
                                                        </div>
                                                    }
                                                })
                                                .collect::<Vec<_>>()}
                                        </div>
                                    </div>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </section>
        </div>
    }
}
