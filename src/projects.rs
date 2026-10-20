#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Project {
    pub name: &'static str,
    pub kind: &'static str,
    pub description: &'static str,
    pub highlights: &'static [&'static str],
    pub url: &'static str,
    pub action: &'static str,
    pub repository: Option<&'static str>,
}

pub const FEATURED: &[Project] = &[
    Project {
        name: "stochastic-rs",
        kind: "Quantitative finance",
        description: "Model stochastic processes, price derivatives and calibrate volatility models in Rust and Python, with SIMD and GPU acceleration.",
        highlights: &["Stochastic processes", "Option pricing", "Risk"],
        url: "https://stochastic.rust-dd.com",
        action: "Explore docs",
        repository: Some("https://github.com/rust-dd/stochastic-rs"),
    },
    Project {
        name: "tako",
        kind: "Web framework",
        description: "One Rust router for HTTP, WebSocket, WebTransport, gRPC and raw sockets. Build services on Tokio or Compio.",
        highlights: &["HTTP/3", "Multi-transport", "Async Rust"],
        url: "https://tako.rust-dd.com",
        action: "Explore docs",
        repository: Some("https://github.com/rust-dd/tako"),
    },
    Project {
        name: "embedded-dht-rs",
        kind: "Embedded systems",
        description: "Read temperature and humidity from DHT11, DHT22 and DHT20 / AHT20 sensors with Rust drivers for embedded hardware.",
        highlights: &["Sensor drivers", "Temperature", "Humidity"],
        url: "https://github.com/rust-dd/embedded-dht-rs",
        action: "View source",
        repository: None,
    },
    Project {
        name: "rsql",
        kind: "Database client",
        description: "A PostgreSQL desktop client built with Rust, Tauri and React. Query data, explore schemas and inspect query plans.",
        highlights: &["PostgreSQL", "EXPLAIN", "Desktop app"],
        url: "https://rsql.rust-dd.com",
        action: "Visit website",
        repository: Some("https://github.com/rust-dd/rsql"),
    },
    Project {
        name: "candding",
        kind: "Machine learning",
        description: "Run dense, sparse and multi-vector embedding models, plus rerankers, in Rust with Candle. No ONNX runtime required.",
        highlights: &["Embeddings", "Reranking", "Candle"],
        url: "https://candding.rust-dd.com",
        action: "Explore docs",
        repository: Some("https://github.com/rust-dd/candding"),
    },
    Project {
        name: "metal-oxide",
        kind: "GPU computing",
        description: "Write compute kernels in Rust and run them on Apple GPUs. A compiler and runtime with typed host bindings for Metal.",
        highlights: &["Apple Silicon", "Rust kernels", "Metal"],
        url: "https://github.com/rust-dd/metal-oxide",
        action: "View source",
        repository: None,
    },
    Project {
        name: "react-native-qdrant-edge",
        kind: "On-device vector search",
        description: "Bring Qdrant vector search into your React Native app, with the engine running directly on iOS and Android.",
        highlights: &["React Native", "Vector search", "iOS + Android"],
        url: "https://github.com/rust-dd/react-native-qdrant-edge",
        action: "View source",
        repository: None,
    },
    Project {
        name: "react-native-scc",
        kind: "Mobile storage",
        description: "Persistent key-value storage for React Native and Expo, powered by Rust. A drop-in MMKV alternative using Nitro Modules.",
        highlights: &["React Native + Expo", "Key-value store", "Rust"],
        url: "https://github.com/rust-dd/react-native-scc",
        action: "View source",
        repository: None,
    },
];

pub const PRODUCTS: &[Project] = &[
    Project {
        name: "turboencode.com",
        kind: "Image & video conversion",
        description: "Convert images and videos with Rust-powered engines. Upload files, choose an output format and download the results without an account.",
        highlights: &["Images", "Video", "No account required"],
        url: "https://turboencode.com",
        action: "Open converter",
        repository: None,
    },
    Project {
        name: "stochasticlab",
        kind: "Quantitative platform",
        description: "Quantitative tools built on stochastic calculus for portfolio optimization, volatility modeling, option pricing and risk analysis.",
        highlights: &["Portfolio optimization", "Volatility", "Risk"],
        url: "https://stochasticlab.cloud",
        action: "Visit website",
        repository: None,
    },
    Project {
        name: "tryrust.org",
        kind: "Learn Rust",
        description: "An interactive Rust tutorial and playground. Work through the language and run your code directly in the browser.",
        highlights: &["Tutorials", "Playground", "In the browser"],
        url: "https://tryrust.org",
        action: "Start learning",
        repository: Some("https://github.com/rust-dd/tryrust.org"),
    },
];

pub const AMNEZIA: Project = Project {
    name: "amnezia-rs",
    kind: "Game restoration",
    description: "Bringing Amnézia, the Hungarian RPG Maker 2000 game, back to life in Rust and Bevy. Original maps, dialogue and turn-based combat in a new runtime.",
    highlights: &["Rust + Bevy", "RPG Maker 2000", "Turn-based combat"],
    url: "https://github.com/rust-dd/amnezia-rs",
    action: "Explore the restoration",
    repository: None,
};

pub const EXPERIMENTS: &[Project] = &[
    AMNEZIA,
    Project {
        name: "doom.rust-dd",
        kind: "Browser game",
        description: "DOOM in the browser, rebuilt with Rust and Bevy on Freedoom.",
        highlights: &["Rust + Bevy", "Freedoom", "In the browser"],
        url: "https://doom.rust-dd.com",
        action: "Play in browser",
        repository: None,
    },
    Project {
        name: "ito",
        kind: "Terminal app",
        description: "Browse, configure and plot the stochastic processes in stochastic-rs from a terminal UI.",
        highlights: &["TUI", "Stochastic processes", "Monte Carlo"],
        url: "https://github.com/rust-dd/ito",
        action: "View source",
        repository: None,
    },
];

pub const PAUSED: &[Project] = &[Project {
    name: "shrtn.ink",
    kind: "URL shortener",
    description: "URL shortening with QR codes, click analytics, a REST API, an MCP server and webhooks. The service is temporarily paused while we work on fixes.",
    highlights: &["Short links", "QR codes", "Analytics"],
    url: "https://shrtn.ink",
    action: "Visit website",
    repository: None,
}];
