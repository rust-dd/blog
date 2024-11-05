# Blog

![example workflow](https://github.com/rust-dd/blog/actions/workflows/rust.yml/badge.svg)

A blog engine written in Rust, powered by SurrealDB. This project is responsible for the [https://rust-dd.com](https://rust-dd.com) blog.

## Running the Project

To get started with running the project locally, follow the steps below. These commands will compile your stylesheets and set up a development server that watches for changes.

First, run the following script to set up your Surreal database:

```bash
./db.sh
```

```bash
cargo install surrealdb-migrations
```

```bash
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

# Deployment

We provide a Dockerfile that allows you to start the blog engine, connecting to an external SurrealDB. The connection details must be defined in the environment variables.

For containerizing the application, build the Docker image with the following command:

```bash
docker build . -t blog
```

Once the image is built, run it with:

```bash
docker run --env-file .env -p 8080:8080 blog
```
