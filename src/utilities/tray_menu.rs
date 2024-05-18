use std::sync::mpsc::{self, Receiver};
use tray_item::TrayItem;

use super::enums::Message;

pub fn init_tray_menu(tray: &mut TrayItem) -> Receiver<Message> {
    let (tx, rx) = mpsc::sync_channel(1);

    // TODO: combine show & hide app into one menu item
    // use https://github.com/olback/tray-item-rs/blob/master/examples/windows-edit-menu-items/src/main.rs
    let show_app_tx = tx.clone();
    tray.add_menu_item("Show app", move || {
        show_app_tx.send(Message::ShowApp).unwrap();
    })
    .unwrap();

    let hide_app_tx = tx.clone();
    tray.add_menu_item("Hide app", move || {
        hide_app_tx.send(Message::HideApp).unwrap();
    })
    .unwrap();

    #[cfg(target_os = "windows")]
    tray.inner_mut().add_separator().unwrap();

    let mount_all_tx = tx.clone();
    tray.add_menu_item("Mount all", move || {
        mount_all_tx.send(Message::MountAll).unwrap();
    })
    .unwrap();

    let unmount_all_tx = tx.clone();
    tray.add_menu_item("Unmount all", move || {
        unmount_all_tx.send(Message::UnmountAll).unwrap();
    })
    .unwrap();

    #[cfg(target_os = "windows")]
    tray.inner_mut().add_separator().unwrap();

    let enable_auto_mount_tx = tx.clone();
    tray.add_menu_item("Enable auto mount", move || {
        enable_auto_mount_tx.send(Message::EnableAutoMount).unwrap();
    })
    .unwrap();

    let disable_auto_mount_tx = tx.clone();
    tray.add_menu_item("Disable auto mount", move || {
        disable_auto_mount_tx
            .send(Message::DisableAutoMount)
            .unwrap();
    })
    .unwrap();

    // #[cfg(target_os = "windows")]
    // tray.inner_mut().add_separator().unwrap();

    // let enable_auto_start_tx = tx.clone();
    // tray.add_menu_item("Enable auto start", move || {
    //     enable_auto_start_tx.send(Message::EnableAutoStart).unwrap();
    // })
    // .unwrap();

    // let disable_auto_start_tx = tx.clone();
    // tray.add_menu_item("Disable auto start", move || {
    //     disable_auto_start_tx
    //         .send(Message::DisableAutoStart)
    //         .unwrap();
    // })
    // .unwrap();

    #[cfg(target_os = "windows")]
    tray.inner_mut().add_separator().unwrap();

    let icon_tx = tx.clone();
    tray.add_menu_item("DriveAF", move || {
        icon_tx.send(Message::Icon).unwrap();
    })
    .unwrap();

    let red_tx = tx.clone();
    tray.add_menu_item("Red", move || {
        red_tx.send(Message::Red).unwrap();
    })
    .unwrap();

    let green_tx = tx.clone();
    tray.add_menu_item("Green", move || {
        green_tx.send(Message::Green).unwrap();
    })
    .unwrap();

    #[cfg(target_os = "windows")]
    tray.inner_mut().add_separator().unwrap();

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    rx
}
