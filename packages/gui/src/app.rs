use eframe::egui;
use egui_dock::{DockArea, Style};

use crate::{domain, menu};

pub(crate) struct MyApp {
    ctx: domain::AppContext,
}

impl Default for MyApp {
    fn default() -> Self {
        let ctx = domain::AppContext::new();
        Self { ctx }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        menu::display_menu(&mut self.ctx, ctx);

        DockArea::new(&mut self.ctx.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut self.ctx.tag_viewer)
    }
}
