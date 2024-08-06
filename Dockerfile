# Stage 1: Build Environment with Rust nightly on Alpine
FROM rustlang/rust:nightly-alpine as builder

# Install required packages
RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen

# Install SASS globally
RUN npm install -g tailwindcss

# Install cargo-leptos
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Create working directory
WORKDIR /work
COPY . .

# Install the required npm dependencies
RUN npm install

# Process Tailwind CSS
RUN npx tailwindcss -i input.css -o ./style/output.css

# Build the application
RUN cargo leptos build --release -vv

# Stage 2: Runtime Environment
FROM rustlang/rust:nightly-alpine as runner

WORKDIR /app

# Add the rustfmt component in the runtime stage
RUN rustup component add rustfmt

# Copy the server binary and site content from the builder stage
COPY --from=builder /work/target/release/tryrust /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

# Set environment variables
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080


# Run the server
CMD ["/app/tryrust"]
