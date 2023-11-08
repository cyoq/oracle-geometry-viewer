# Oracle Geometry Viewer GUI

Built with [egui](https://github.com/emilk/egui/tree/master).

To run the project:

```bash
cargo run
```

Or to release:

```bash
cargo run --release
```

To build Windows application from Mac ARM computer([Source](https://stackoverflow.com/a/67063394)):

1. Install mingw-w64: `brew install mingw-w64`
2. Install target: `rustup target add x86_64-pc-windows-gnu`
3. Install toolchain: `rustup toolchain install stable-x86_64-pc-windows-gnu`
4. Build the project: `cargo build --release --target=x86_64-pc-windows-gnu`