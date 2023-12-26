# RustyNight
A generated animation of a winter night using [rust](https://www.rust-lang.org/)

# Building 

## Running natively

```bash

cargo run --release

```
(press space to start the animation)

## Running as web assembly

RustyNight can be compiled to web assembly using [cargo-run-wasm](https://github.com/rukai/cargo-run-wasm)

```bash
cargo run-wasm --release --package rusty_night
```

this opens a local webserver at `http://localhost:8080` where the animation can be viewed. (Press space to start the animation)

