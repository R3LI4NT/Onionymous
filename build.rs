fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        if std::path::Path::new("assets/logo.ico").exists() {
            res.set_icon("assets/logo.ico");
        }
        res.set("ProductName", "Onionymous");
        res.set("FileDescription", "Privacy-first Tor routing application");
        res.set("LegalCopyright", "Copyright © 2026 Onionymous");
        let _ = res.compile();
    }
}
