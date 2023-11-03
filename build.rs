#[cfg(target_os = "windows")]
extern crate windres;

#[cfg(target_os = "windows")]
fn main() {
    windres::Build::new().compile("tray-resource.rc").unwrap();
    println!("cargo:rerun-if-changed=assets/app-icon.ico");
}

