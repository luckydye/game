rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./dist/ --target web ./target/wasm32-unknown-unknown/release/game.wasm
