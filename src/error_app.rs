use eframe::App;
use egui::CentralPanel;

#[derive(Debug)]
pub struct ErrorApp {
    pub is_platform_supported: bool,
    pub platform: String,
    pub missing_dependencies: Vec<String>,
}

impl App for ErrorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Error");

            if self.is_platform_supported {
                ui.label("This app only supports Windows, Linux, and MacOS.");
                ui.label("Other platforms will come soon!");
                ui.label(format!("Your platform is: {}", self.platform));
            }

            if !self.missing_dependencies.is_empty() {
                ui.label("The following dependencies are missing:");
                for dependency in &self.missing_dependencies {
                    ui.label(format!("- {}", dependency));
                }
                ui.label("Please install the missing dependencies and try again.");
            }
        });
    }
}
