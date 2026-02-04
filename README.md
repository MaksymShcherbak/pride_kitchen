# Pride Kitchen 🍴

![](index.png)

Mix & Match Pride Flags in this interactive app, creating unique and colorful combinations!

## Features

- 🏳‍🌈 **20+ flags** for various identities
- 🎨 Customizable options
- 💾 Download your creations as PNG or SVG.

## Tech Stack

- 🦀 [Rust](https://rust-lang.org/) Programming Language.
- 🧬 [Dioxus](https://dioxuslabs.com/) UI framework (Rust-based).
- 🌐 [WASM](https://webassembly.org/) for web deployment.

## Build Requirements

If you want to build and run the project locally, ensure you have the following installed:

- Rust toolchain (stable) with `cargo`.

- Dioxus CLI.

```bash
cargo install dioxus-cli --version 0.7.3
```

- The `wasm32-unknown-unknown` target.

```bash
rustup target add wasm32-unknown-unknown
```

## Running the App

Run the web server:

```bash
dx serve
```

## Deploy

To build the project for deployment:

```bash
dx build --release
```
