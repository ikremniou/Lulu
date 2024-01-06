mod open;
mod save;
use eframe::egui;

use crate::domain::AppContext;

pub(crate) fn display_menu(app_context: &mut AppContext, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open File...").clicked() {
                    open::open_file(app_context)
                }
                if ui.button("Open Folder...").clicked() {
                    open::open_folder(app_context);
                }
                if ui.button("Save").clicked() {
                    save::save(app_context);
                }
                if ui.button("Save As...").clicked() {
                    save::save_as(app_context);
                }
                if ui.button("Exit").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
            ui.menu_button("Help", |ui| if ui.button("About").clicked() {})
        });
    });
}
