#[cfg(target_os = "windows")]
fn main() {
    extern crate winres;

    let mut res = winres::WindowsResource::new();

    res.set_icon("assets/DriveFUSE.ico")
        .set_icon_with_id("assets/DriveFUSE.ico", "app-icon");

    res.compile().expect("Failed to compile resources");
}

#[cfg(target_os = "linux")]
fn main() {
    println!("cargo:rerun-if-changed=assets/DriveFUSE.ico");
}

#[cfg(target_os = "macos")]
fn main() {
    println!("cargo:rerun-if-changed=assets/DriveFUSE.ico");
}
