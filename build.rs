#[cfg(windows)]
fn main() {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("assets/B0XX.ico");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
