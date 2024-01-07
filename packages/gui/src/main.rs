use eframe::egui;
mod app;
mod domain;
mod error;
mod menu;
mod components;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Lulu Disassembler",
        options,
        Box::new(|_| Box::<app::MyApp>::default()),
    )
}
