use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native("Lulu GUI", options, Box::new(|_| Box::<MyApp>::default()))
}

struct MyApp {
    pub text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text: "Hello World!".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { text } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {}
                    if ui.button("Save").clicked() {}
                    if ui.button("Exit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Help", |ui| if ui.button("About").clicked() {})
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TextEdit::multiline(text)
                .desired_rows(10)
                .font(egui::TextStyle::Monospace)
                .code_editor()
                .desired_width(f32::INFINITY)
                .lock_focus(true)
                .show(ui);
        });
    }
}
