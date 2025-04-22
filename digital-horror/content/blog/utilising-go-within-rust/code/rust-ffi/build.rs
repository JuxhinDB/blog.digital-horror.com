// build.rs
fn main() {
    // If libwapp.so is in the project root:
    println!(
        "cargo:rustc-link-search=native={}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    );
    // Link to “-lgoffi”
    println!("cargo:rustc-link-lib=dylib=goffi");
}
