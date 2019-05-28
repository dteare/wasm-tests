# WASM tests

Simple projects for learning about WASM features firsthand while reading docs and blog posts.

## Cargo expand

The `cargo-expand` folder demonstrates how to use [cargo expand](https://github.com/dtolnay/cargo-expand) to see what the `wasm_bindgen` macro expands to. 

* Install with `cargo install cargo-expand`
* Run `make expand`
* Look at [expanded.rs](./cargo-expand/expanded.rs) to see the generated stack machine. 🙂

HT to [Rust and JavaScript Interop](https://blog.ryanlevick.com/posts/wasm-bindgen-interop/) for teaching me this one. 