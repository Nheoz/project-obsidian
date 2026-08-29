use std::env;
use std::path::Path;

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let res_path = Path::new(&manifest_dir).join("app.res");
        if res_path.exists() {
            println!("cargo:rustc-link-arg-bins={}", res_path.display());
        }
    }
}
