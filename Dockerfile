FROM rust:1.46.0-slim-buster as build
## Stage 1

RUN apt-get update && apt-get install -y pkg-config libssl-dev curl
RUN cargo install wasm-pack

RUN mkdir -p /build_dir/static
WORKDIR /build_dir

COPY ./Cargo.lock /build_dir/Cargo.lock
COPY ./Cargo.toml /build_dir/Cargo.toml
COPY ./src /build_dir/src

RUN curl https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css > static/bootstrap_4.5.2_.min.css
RUN curl https://code.jquery.com/jquery-3.5.1.slim.min.js > static/jquery-3.5.1.slim.min.js
RUN curl https://cdn.jsdelivr.net/npm/popper.js@1.16.1/dist/umd/popper.min.js > static/popper.min.js
RUN curl https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/js/bootstrap.min.js > static/bootstrap.min.js
RUN wasm-pack build --target web --out-name wasm --out-dir ./static

## Stage 2
FROM manimaul/jacuzzi:0.1.1
COPY --from=build /build_dir/static /www
CMD ["/www"]
