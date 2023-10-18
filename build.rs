use windres::Build;

fn main() {
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        Build::new().compile("tray-resource.rc").unwrap();
        println!("cargo:rerun-if-changed=assets/app-icon.ico");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
