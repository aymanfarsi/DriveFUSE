use std::sync::mpsc::{self, Receiver};
use tray_item::TrayItem;

use super::enums::Message;

pub fn init_tray_menu(tray: &mut TrayItem) -> Receiver<Message> {
    let (tx, rx) = mpsc::sync_channel(1);

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

    tray.inner_mut().add_separator().unwrap();

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    rx
}
