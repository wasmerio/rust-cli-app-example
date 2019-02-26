# rust-cli-app-example

A demonstration application. This shows how one can create command-line apps in WebAssembly with Emscripten, Rust, and Wasmer.

# Build

Install wasmer with the [installation instructions][wasmer_install] or [clone the repo][wasmer_git_repo]. 

Install and setup Emscripten in the CLI app build environment. 
See the [emscripten installation instructions][emscripten_install]. 
Run the setup script wherever running `cargo` commands.
Build the project in debug mode (the default).

```
> cargo build
```

# Run

Run with Wasmer:
```
> wasmer run ./target/wasm32-unknown-emscripten/debug/rust_cli_app_example.wasm
```


[wasmer_install]: https://wasmer.io
[wasmer_git_repo]: https://github.com/wasmerio/wasmer
[emscripten_install]: https://emscripten.org/docs/getting_started/downloads.html
