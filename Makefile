win-web:
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "thunder" ./target/wasm32-unknown-unknown/release/thunder.wasm
	robocopy "assets" "out\assets" /E /XO

linux-web:
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "thunder" ./target/wasm32-unknown-unknown/release/thunder.wasm
	cp -r assets out/assets