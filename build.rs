fn main() {
    println!("cargo:rerun-if-changed=obsidian.manifest");

    // We only embed the manifest for Windows builds
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_manifest_file("obsidian.manifest");
        res.compile().unwrap();
    }
}
