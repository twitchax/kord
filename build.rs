fn main() {
    if cfg!(windows) {
        println!("cargo:rustc-cfg=host_family_windows");
    }
    if cfg!(unix) {
        println!("cargo:rustc-cfg=host_family_unix");
    }
}