# Setup Steps
```
cargo new --lib willkamp-com
cargo install wasm-pack
cargo install jacuzzi
```


# Run
```
wasm-pack build --target web --out-name wasm --out-dir ./static
jacuzzi ./static
```
