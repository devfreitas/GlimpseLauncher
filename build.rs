use std::env;

fn main() {
    println!("cargo:rerun-if-changed=public/icone.ico");

    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("public/icone.ico");
        res.set("FileDescription", "Glimpse Launcher");
        res.set("ProductName", "Glimpse Launcher");
        res.set("OriginalFilename", "glimpse_launcher.exe");
        res.compile().unwrap();
    }
}
