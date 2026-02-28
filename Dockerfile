# Stage 1: Build
FROM rustlang/rust:nightly-alpine AS builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen clang

RUN cargo install dioxus-cli --locked
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN npm install
RUN dx bundle --web --release

# Stage 2: Runtime
FROM rustlang/rust:nightly-alpine AS runner

WORKDIR /app

# Dioxus fullstack bundle output
COPY --from=builder /work/target/dx/blog/release/web /app

ENV RUST_LOG="info"
ENV IP="0.0.0.0"
ENV PORT="8080"

EXPOSE 8080
CMD ["/app/server"]
