# Setup Steps
```
cargo new --lib willkamp-com
cargo install wasm-pack
cargo install jacuzzi
```

# Optional (Bootstrap)
```
curl https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css > static/bootstrap_4.5.2_.min.css
curl https://code.jquery.com/jquery-3.5.1.slim.min.js > static/jquery-3.5.1.slim.min.js
curl https://cdn.jsdelivr.net/npm/popper.js@1.16.1/dist/umd/popper.min.js > static/popper.min.js
curl https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/js/bootstrap.min.js > static/bootstrap.min.js 
```

# Run
```
wasm-pack build --target web --out-name wasm --out-dir ./static
jacuzzi ./static
```
