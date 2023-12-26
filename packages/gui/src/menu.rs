use eframe::egui;

pub(crate) fn display_menu(ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open File...").clicked() {}
                if ui.button("Open Folder...").clicked() {}
                if ui.button("Save").clicked() {}
                if ui.button("Save As...").clicked() {}
                if ui.button("Exit").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
            ui.menu_button("Help", |ui| if ui.button("About").clicked() {})
        });
    });
}
