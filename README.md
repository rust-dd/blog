# Blog

A blog engine written in Rust, powered by SurrealDB. This project runs [https://rust-dd.com](https://rust-dd.com).

## Stack

- Dioxus `0.7.x` (fullstack + router)
- Axum `0.8`
- SurrealDB `3.x`
- TailwindCSS

## Local Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started/)
- [SurrealDB](https://surrealdb.com/install)
- [Node.js / npm](https://nodejs.org/)

Install Dioxus CLI:

```bash
cargo install dioxus-cli
```

Prepare the database (schema lives in `database/schema/`, managed by [surrealkit](https://github.com/surrealdb/surrealkit)):

```bash
./db.sh
cargo binstall surrealkit
surrealkit sync
```

Install frontend tooling:

```bash
npm install
```

Run Dioxus fullstack dev server with Subsecond hotpatch:

```bash
dx serve --web --hotpatch
```

`dx` automatically compiles Tailwind when `tailwind.css` exists in the project root.

## Build

Bundle app:

```bash
dx bundle --web --release
```
