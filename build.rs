#[cfg(target_os = "windows")]
fn main() {
    extern crate winres;

    let mut res = winres::WindowsResource::new();

    res.set_icon("assets/DriveFUSE.ico")
        .set_icon_with_id("assets/DriveFUSE.ico", "app-icon");

    res.compile().unwrap();
}

#[cfg(target_os = "linux")]
fn main() {
    println!("cargo:rerun-if-changed=assets/DriveAF.ico");
}

#[cfg(target_os = "macos")]
fn main() {
    println!("cargo:rerun-if-changed=assets/DriveAF.ico");
}
