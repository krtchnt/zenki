# Zenki

♨️ A small web application simulating a game distribution service (e.g. Steam).

> [!IMPORTANT]
> This project is developed under the *01204351 Database Systems I* course of **Department of Computer Engineering**, **Faculity of Engineering**, **Kasetsart University**.

> **Project Developers**:
> * *กฤชณัท ธนพิพัฒนศิริ, Kritchanat Thanapiphatsiri (6610501955)*
> * *ธนภัทร กาญจนรุจิวุฒิ, Tanapatr Karnjanarujivut (6610505403)*
> * *วรุตม์ มาศสุวรรณ,   Warut Massuwan (6610505560)*

<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

This project was based on a template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool using [Axum](https://github.com/tokio-rs/axum).

## Installation

If you are running a Nix system, simply run `nix develop --impure`. Otherwise, follow the instructions below.

First, Install `cargo-leptos` via running:

```bash
cargo install cargo-leptos --locked
```

Then clone the repository, and change the working directory into `zenki`.

You will also need a running PostgreSQL server. To apply migrations, `cd` to `database`, then run `cargo install sqlx-cli` as well as `cargo sqlx database setup`.

Create an `.env` file containing the database connection string like so. Edit the angle bracketed segments to your database's parameters accordingly.
```
DATABASE_URL='postgresql://<username>:<password>@<host>:<port>/<database_name>'
```

and everything should be ready.

## Running your project

```bash
cargo leptos watch
```

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future
5. Run `npm install` in end2end subdirectory before test

## Compiling for Release
```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site

## Testing Your Project
```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.
Tests are located in end2end/tests directory.

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
zenki
site/
```
Set the following environment variables (updating for your project as needed):
```sh
export LEPTOS_OUTPUT_NAME="zenki"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

## Licensing

This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.
