use rclone_app::{
    utils::{enums::Message, tray_menu::init_tray_menu},
    RcloneApp,
};
use tray_item::{IconSource, TrayItem};

fn main() {
    let mut tray =
        TrayItem::new("RcloneApp Tray", IconSource::Resource("green-icon-file")).unwrap();

    let rx = init_tray_menu(&mut tray);
    // let mut tx_main: Option<SyncSender<Message>> = None;

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                println!("Quit");
                // if let Some(tx) = tx_main.clone() {
                //     tx.send(Message::Quit).unwrap();
                // } else {
                //     println!("[Quit] No app is opened");
                // }
                break;
            }
            Ok(Message::Red) => {
                // if let Some(tx) = tx_main.clone() {
                //     tx.send(Message::Red).unwrap();
                // } else {
                //     println!("[Red] No app is opened");
                // }
                tray.set_icon(IconSource::Resource("red-icon-file"))
                    .unwrap();
                println!("Red");
            }
            Ok(Message::Green) => {
                // if let Some(tx) = tx_main.clone() {
                //     tx.send(Message::Green).unwrap();
                // } else {
                //     println!("[Green] No app is opened");
                // }
                tray.set_icon(IconSource::Resource("green-icon-file"))
                    .unwrap();
                println!("Green");
            }
            Ok(Message::ShowApp) => {
                // if is_hidden {
                //     println!("[ShowApp] Opening app");
                //     is_hidden = false;
                //     let (app, tx) = RcloneApp::new();
                //     tx_main = Some(tx);
                //     let native_options = eframe::NativeOptions::default();
                //     let _ = eframe::run_native(
                //         "Rclone App",
                //         native_options,
                //         Box::new(|_cc| Box::new(app)),
                //     );
                // } else {
                //     println!("[ShowApp] App is already opened");
                // }
                let (app, _) = RcloneApp::new();
                let native_options = eframe::NativeOptions {
                    centered: true,
                    initial_window_size: Some(egui::Vec2::new(400.0, 200.0)),
                    ..Default::default()
                };
                let _ =
                    eframe::run_native("Rclone App", native_options, Box::new(|_cc| Box::new(app)));
            }
            Err(_) => {
                println!("Error");
                break;
            }
        }
    }
}
