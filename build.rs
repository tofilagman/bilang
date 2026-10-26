// Puts the app icon and version info into bilang.exe on Windows.
fn main() {
    #[cfg(windows)]
    {
        println!("cargo:rerun-if-changed=packaging/windows/bilang.ico");
        let mut res = winresource::WindowsResource::new();
        res.set_icon("packaging/windows/bilang.ico");
        res.set("ProductName", "Bilang");
        res.set("FileDescription", "Bilang: a math adventure for kids");
        res.compile().expect("embedding the Windows icon");
    }
}
