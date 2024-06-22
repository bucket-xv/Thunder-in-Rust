win-web:
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --no-typescript --target web --out-dir ./docs/ --out-name "thunder" ./target/wasm32-unknown-unknown/release/thunder.wasm
	robocopy "assets" "docs\assets" /E /XO

linux-web:
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --no-typescript --target web --out-dir ./docs/ --out-name "thunder" ./target/wasm32-unknown-unknown/release/thunder.wasm
	cp -r assets docs/assets