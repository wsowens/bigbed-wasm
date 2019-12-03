default: web

web: optimized
	cp optimized/bigbed_wasm.js optimized/bigbed_wasm_bg.wasm -t web

optimized: wasm32
	wasm-bindgen target/wasm32-unknown-unknown/debug/bigbed_wasm.wasm --out-dir optimized --target web

wasm32:
	cargo +nightly build --target wasm32-unknown-unknown

clean:
	rm -f web/bigbed_wasm_bg.wasm web/bigbed_wasm.js
	rm -rf optimized/
	rm -f target/wasm32-unknown-unknown/debug/*wasm target/wasm32-unknown-unknown/debug/*.d