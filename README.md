# Blog

A blog engine written in Rust, powered by SurrealDB. This project is responsible for the [https://rust-dd.com](https://rust-dd.com) blog.

## Running the Project

To get started with running the project locally, follow the steps below. These commands will compile your stylesheets and set up a development server that watches for changes.

First, run the following script to set up your Surreal database:

```bash
./db.sh
```

```
cargo install surrealdb-migrations

```
surrealdb-migrations apply
```

Next, compile the CSS using TailwindCSS. This command will watch for changes in your CSS files and recompile them automatically:


```bash
npx tailwindcss -i input.css -o ./style/output.css --watch
```

Finally, run the Leptos development server. This will watch for changes in your project and automatically reload:


```bash
cargo leptos watch
```
