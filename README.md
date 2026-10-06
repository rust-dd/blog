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

## Blog admin

Open `/admin` to write and manage posts. The editor supports Markdown with a live preview, selectable authors, topics, tags, hosted image URLs, draft saves, publication, and deletion. Save preserves a post's publication status; publishing is a separate action.

Generate your server credentials with hidden password input:

```bash
python3 scripts/configure_admin.py
```

Set the generated `ADMIN_PASSWORD_HASH` and `ADMIN_SESSION_SECRET` in Railway's service variables. Set `ADMIN_ORIGIN=https://rust-dd.com` (the default). Changing either credential revokes all existing sessions. Without valid credentials, admin access stays disabled.

For local development, set `ADMIN_ORIGIN=http://127.0.0.1:8080` to match your dev server URL. HTTPS and secure cookies are required for all other hosts. Keep credentials out of git; local `*.env` files are ignored. `ADMIN_TRUST_PROXY=true` enables per-client throttling behind a trusted reverse proxy; enable it only when the app cannot be reached directly and the proxy appends the real client IP to `X-Forwarded-For`.

Apply the schema before deploying the admin: `post.first_published_at` is an optional datetime used to preserve the original publication date when an article is unpublished and republished. Use the existing surrealkit workflow (`sync --dry-run --no-prune`, then `sync --no-prune` and `apply`). Existing published articles retain their date.

Admin pages and APIs require a valid session, use `no-store`, and are excluded from indexing. Public post routes and APIs expose published posts only. Preview HTML runs in a sandboxed iframe with scripts disabled.

Run the shared and server checks:

```bash
cargo test --locked
cargo test --locked --features server
cargo check --locked --features web --target wasm32-unknown-unknown
node --test scripts/admin_navigation.test.cjs
```

Server tests start isolated local SurrealDB instances and require the `surreal` binary. They do not read or modify the production database.
