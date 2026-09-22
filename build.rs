#[cfg(windows)]
fn main() {
    println!("cargo:rerun-if-changed=src/assets/icon.ico");
    println!("cargo:rerun-if-changed=build.rs");
    let mut res = winres::WindowsResource::new();
    res.set_icon("src/assets/icon.ico");
    if let Err(e) = res.compile() {
        eprintln!("[build.rs] Warning: Failed to compile windows resource: {}", e);
    }
}

#[cfg(not(windows))]
fn main() {}
