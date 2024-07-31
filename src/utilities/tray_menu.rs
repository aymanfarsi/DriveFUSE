#[cfg(not(target_os = "linux"))]
use {
    super::enums::Message,
    tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver},
    tray_item::TrayItem,
};

#[cfg(target_os = "linux")]
use tray_icon::menu::{Menu, MenuItem, PredefinedMenuItem};

#[cfg(target_os = "linux")]
pub fn init_tray_menu(menu: &mut Menu) {
    use tray_icon::menu::AboutMetadataBuilder;

    let toml_lines = include_str!("../../Cargo.toml").lines();

    let authors = toml_lines
        .clone()
        .find(|line| line.starts_with("authors"))
        .expect("Failed to find authors line in Cargo.toml")
        .replace("authors = [\"", "")
        .replace("\"]", "")
        .split("\", \"")
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();

    let description = toml_lines
        .clone()
        .find(|line| line.starts_with("description"))
        .expect("Failed to find description line in Cargo.toml")
        .replace("description = \"", "")
        .replace("\"", "");

    let version = toml_lines
        .clone()
        .find(|line| line.starts_with("version"))
        .expect("Failed to find version line in Cargo.toml")
        .replace("version = \"", "")
        .replace("\"", "");

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load(
            std::io::Cursor::new(include_bytes!("../../assets/drivefuse.png")),
            image::ImageFormat::Png,
        )
        .expect("Failed to open icon path")
        .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let app_icon = tray_icon::menu::Icon::from_rgba(icon_rgba, icon_width, icon_height)
        .expect("Failed to open icon");

    menu.append_items(&[
        &MenuItem::with_id("show_app", "Show app", true, None),
        &MenuItem::with_id("hide_app", "Hide app", true, None),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id("mount_all", "Mount all", true, None),
        &MenuItem::with_id("unmount_all", "Unmount all", true, None),
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::about(
            Some("About DriveFUSE"),
            Some(
                AboutMetadataBuilder::new()
                    .authors(Some(authors))
                    .comments(Some(description))
                    .icon(Some(app_icon))
                    .name(Some("DriveFUSE"))
                    .version(Some(version))
                    .website(Some("https://www.github.com/aymanfarsi/DriveFUSE"))
                    .website_label(Some("DriveFUSE GitHub"))
                    .build(),
            ),
        ),
        &MenuItem::with_id("quit", "Quit", true, None),
    ])
    .expect("Failed to append items to menu");
}

#[cfg(not(target_os = "linux"))]
pub fn init_tray_menu(tray: &mut TrayItem) -> UnboundedReceiver<Message> {
    let (tx, rx) = unbounded_channel();

    // TODO: combine show & hide app into one menu item
    // use https://github.com/olback/tray-item-rs/blob/master/examples/windows-edit-menu-items/src/main.rs
    let show_app_tx = tx.clone();
    tray.add_menu_item("Show app", move || {
        show_app_tx
            .send(Message::ShowApp)
            .expect("Failed to send ShowApp message");
    })
    .expect("Failed to add Show app menu item");

    let hide_app_tx = tx.clone();
    tray.add_menu_item("Hide app", move || {
        hide_app_tx
            .send(Message::HideApp)
            .expect("Failed to send HideApp message");
    })
    .expect("Failed to add Hide app menu item");

    #[cfg(target_os = "windows")]
    tray.inner_mut()
        .add_separator()
        .expect("Failed to add separator");

    let mount_all_tx = tx.clone();
    tray.add_menu_item("Mount all", move || {
        mount_all_tx
            .send(Message::MountAll)
            .expect("Failed to send MountAll message");
    })
    .expect("Failed to add Mount all menu item");

    let unmount_all_tx = tx.clone();
    tray.add_menu_item("Unmount all", move || {
        unmount_all_tx
            .send(Message::UnmountAll)
            .expect("Failed to send UnmountAll message");
    })
    .expect("Failed to add Unmount all menu item");

    #[cfg(target_os = "windows")]
    tray.inner_mut()
        .add_separator()
        .expect("Failed to add separator");

    let icon_tx = tx.clone();
    tray.add_menu_item("DriveFUSE", move || {
        icon_tx
            .send(Message::Icon)
            .expect("Failed to send Icon message");
    })
    .expect("Failed to add DriveFUSE menu item");

    #[cfg(target_os = "windows")]
    tray.inner_mut()
        .add_separator()
        .expect("Failed to add separator");

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx
            .send(Message::Quit)
            .expect("Failed to send Quit message");
    })
    .expect("Failed to add Quit menu item");

    rx
}
