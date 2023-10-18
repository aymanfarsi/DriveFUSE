use std::sync::mpsc::{self, Receiver};
use tray_item::TrayItem;

use super::enums::Message;

pub fn init_tray_menu(tray: &mut TrayItem) -> Receiver<Message> {
    // tray.add_label("Tray Label").unwrap();

    // tray.add_menu_item("Hello", || {
    //     println!("Hello!");
    // })
    // .unwrap();

    // tray.inner_mut().add_separator().unwrap();

    let (tx, rx) = mpsc::sync_channel(1);

    let show_app_tx = tx.clone();
    tray.add_menu_item("Show app (click once)", move || {
        show_app_tx.send(Message::ShowApp).unwrap();
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
    tray.add_menu_item("Quit (close app first)", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    rx
}
