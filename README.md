# rust-3d

## For AI readers
- [WorkspaceLayout.md](./WorkspaceLayout.md)を読んで、階層と役割を理解してください。

## Examples
```
cargo run -p integration_min_desktop

wasm-pack build examples/web/integration_min --release --target web --out-dir pkg
cd examples/web/integration_min
python -m http.server 8080
```