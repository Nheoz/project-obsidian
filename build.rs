fn main() {
    println!("cargo:rerun-if-changed=obsidian.manifest");

    // We only embed the UAC requireAdministrator manifest for Release builds.
    // If embedded in debug/test, `cargo test` will fail with OS error 740.
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows"
        && std::env::var("PROFILE").unwrap_or_default() == "release" 
    {
        let mut res = winres::WindowsResource::new();
        res.set_manifest_file("obsidian.manifest");
        res.compile().unwrap();
    }
}
