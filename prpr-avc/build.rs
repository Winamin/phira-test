use std::path::Path;

fn main() {
    let libs_dir = std::env::var("PRPR_AVC_LIBS").unwrap_or_else(|_| format!("{}/static-lib", std::env::var("CARGO_MANIFEST_DIR").unwrap()));
    let libs_path = Path::new(&libs_dir).join(std::env::var("TARGET").unwrap());
    let libs_path = libs_path.display();
    println!("cargo:rustc-link-search={libs_path}");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rerun-if-changed={libs_path}");
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-lib=static=swscale");
    println!("cargo:rustc-link-lib=static=avformat");
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=Xext");
    println!("cargo:rustc-link-lib=Xv");
}
