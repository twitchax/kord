fn main() {
    if cfg!(wasm) {
        println!("cargo:rustc-cfg=wasm");
    }
}
