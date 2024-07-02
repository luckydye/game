echo "Add wasm32-unknown-unknown target..."
rustup target add wasm32-unknown-unknown
echo "Building wasm..."
cargo build --release --target wasm32-unknown-unknown
echo "Packing wasm..."
wasm-bindgen --out-dir ./dist/ --target web ./target/wasm32-unknown-unknown/release/game.wasm
