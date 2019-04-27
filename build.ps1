cargo +nightly build --target wasm32-unknown-unknown --release

if ($? -eq $true) {
  wasm-gc ./target/wasm32-unknown-unknown/release/honest.wasm ./public/wasm/honest.wasm
}
