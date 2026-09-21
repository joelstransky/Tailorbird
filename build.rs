#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("src/assets/icon.ico");
    if let Err(e) = res.compile() {
        eprintln!("[build.rs] Warning: Failed to compile windows resource: {}", e);
    }
}

#[cfg(not(windows))]
fn main() {}
