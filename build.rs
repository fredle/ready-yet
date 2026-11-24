fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/app.ico");
        res.set("FileDescription", "Ready Yet");
        res.set("ProductName", "Ready Yet");
        res.set("OriginalFilename", "ready_yet.exe");
        if let Err(e) = res.compile() {
            // An icon is cosmetic - never fail the build over it.
            println!("cargo:warning=icon not embedded: {e}");
        }
    }
}
