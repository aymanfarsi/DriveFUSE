use tokio::sync::mpsc::{self, Receiver};
use tray_item::TrayItem;

use super::enums::Message;

pub fn init_tray_menu(tray: &mut TrayItem) -> Receiver<Message> {
    let (tx, rx) = mpsc::channel(10);

    // TODO: combine show & hide app into one menu item
    // use https://github.com/olback/tray-item-rs/blob/master/examples/windows-edit-menu-items/src/main.rs
    let show_app_tx = tx.clone();
    tray.add_menu_item("Show app", move || {
        show_app_tx
            .try_send(Message::ShowApp)
            .expect("Failed to send ShowApp message");
    })
    .expect("Failed to add Show app menu item");

    let hide_app_tx = tx.clone();
    tray.add_menu_item("Hide app", move || {
        hide_app_tx
            .try_send(Message::HideApp)
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
            .try_send(Message::MountAll)
            .expect("Failed to send MountAll message");
    })
    .expect("Failed to add Mount all menu item");

    let unmount_all_tx = tx.clone();
    tray.add_menu_item("Unmount all", move || {
        unmount_all_tx
            .try_send(Message::UnmountAll)
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
            .try_send(Message::Icon)
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
            .try_send(Message::Quit)
            .expect("Failed to send Quit message");
    })
    .expect("Failed to add Quit menu item");

    rx
}
