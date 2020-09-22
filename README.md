# Setup Steps
```
cargo new --lib willkamp-com
curl https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css > static/bootstrap_4.5.2_.min.css
cargo install wasm-pack
cargo install jacuzzi
```


# Run
```
wasm-pack build --target web --out-name wasm --out-dir ./static
jacuzzi ./static
```
