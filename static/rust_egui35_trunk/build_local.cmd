REM cargo build -r
REM trunk build --release
REM check dist/index.html
trunk build --release --public-url . --no-default-features
cargo build --release --target wasm32-unknown-unknown

REM C:\Users\Andrey\AppData\Local\trunkrs\trunk\cache\wasm-bindgen-0.2.126\wasm-bindgen.exe  target/wasm32-unknown-unknown/release/rust_hello.wasm --out-dir dist --target web --no-modules
C:\Users\Andrey\AppData\Local\trunkrs\trunk\cache\wasm-bindgen-0.2.126\wasm-bindgen.exe target/wasm32-unknown-unknown/release/rust_hello.wasm    --out-dir dist  --target no-modules
