fn main() {
    #[cfg(windows)]
    {
        use std::io::Write;
        let mut res = winres::WindowsResource::new();
        res.set_icon("../../res/icon.ico")
            .set_language(winapi::um::winnt::MAKELANGID(
                winapi::um::winnt::LANG_ENGLISH,
                winapi::um::winnt::SUBLANG_ENGLISH_US,
            ))
            .set_manifest_file("../../res/manifest.xml")
            .set("ProductName", "Reyes Tech Solutions")
            .set("FileDescription", "Reyes Tech Solutions Remote Desktop")
            .set("CompanyName", "Reyes Tech Solutions")
            .set("LegalCopyright", "Copyright © 2026 Reyes Tech Solutions. Basado en RustDesk © Purslane Ltd. y contribuidores, licencia AGPLv3.");
        match res.compile() {
            Err(e) => {
                write!(std::io::stderr(), "{}", e).unwrap();
                std::process::exit(1);
            }
            Ok(_) => {}
        }
    }
}
